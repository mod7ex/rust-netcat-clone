use std::process::Stdio;
use tokio::{io::{AsyncRead, AsyncWrite}, process::Command};

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

    let stdin = child.stdin.unwrap();
    let stdout = child.stdout.unwrap();
    let stderr = child.stderr.unwrap();

    let mut stdin_buffer = [0; 265];
    let mut network_in_buffer = [0; 256];

    let mut stdin = stdin();

    loop {
        select! {
            res = stdin.read(&mut stdin_buffer) => {
                if let Ok(amount) = res {
                    socket
                        .send(&stdin_buffer[0..amount])
                        .await
                        .expect("failed to send data");
                } else {
                    res.unwrap();
                }
            }

            res = socket.recv(&mut network_in_buffer) => {
                if let Ok(amount) = res {
                    stdout()
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