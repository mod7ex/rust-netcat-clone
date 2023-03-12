use tokio::io::{AsyncRead, AsyncWrite};

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