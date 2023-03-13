use std::process::Stdio;
use tokio::{
    io::{
        AsyncReadExt,
        AsyncWriteExt,
        AsyncRead,
        AsyncWrite
    }, 
    process::Command, 
    select
};

pub async fn read_write<R, W>(mut reader: R, mut writer: W)
    where
        R: AsyncRead + Unpin + Sized + Send + 'static,
        W: AsyncWrite + Unpin + Sized + Send + 'static,
{
    let read = tokio::spawn(async move {
        tokio::io::copy(&mut reader, &mut tokio::io::stdout()).await.unwrap();
    });

    let write = tokio::spawn(async move {
        tokio::io::copy(&mut tokio::io::stdin(), &mut writer).await.unwrap();
    });

    tokio::select! {
        _ = read => {}
        _ = write => {}
    };
}

pub async fn read_write_exec<R, W>(mut reader: R, mut writer: W, cmd: &str)
    where
        R: AsyncRead + Unpin + Sized + Send + 'static,
        W: AsyncWrite + Unpin + Sized + Send + 'static,
{
    let child = Command::new(cmd)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let mut stdin = child.stdin.unwrap();
    let mut stdout = child.stdout.unwrap();
    let mut stderr = child.stderr.unwrap();

    let mut stdout_buffer = [0; 512];
    let mut stderr_buffer = [0; 512];
    let mut network_in_buffer = [0; 512];

    let mut active = true;

    while active {
        select! {
            res = stdout.read(&mut stdout_buffer) => {
                if let Ok(amount) = res {
                    if amount != 0 {
                        writer
                            .write(&stdout_buffer[0..amount])
                            .await
                            .expect("failed to write data");
                    } else {
                        active = false;
                    }
                } else {
                    res.unwrap();
                }
            }

            res = stderr.read(&mut stderr_buffer) => {
                if let Ok(amount) = res {
                    if amount != 0 {
                        writer
                            .write(&stderr_buffer[0..amount])
                            .await
                            .expect("failed to write data");
                    } else {
                        active = false;
                    }
                } else {
                    res.unwrap();
                }
            }

            res = reader.read(&mut network_in_buffer) => {
                if let Ok(amount) = res {
                    stdin
                        .write(&network_in_buffer[0..amount])
                        .await
                        .expect("failed to write to stdout");
                } else {
                    res.unwrap();
                }
            }
        }
    }
}