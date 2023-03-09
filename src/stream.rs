/*
async fn run() -> Result<(), String> {
    let mut stdin = tokio::io::stdin();
    let mut stdout = tokio::io::stdout();

    tokio::spawn(async move {
        tokio::io::copy(&mut stdin, &mut stdout).await.unwrap();
    }).await.unwrap();

    Ok(())
}
*/

pub async fn client() -> Result<(), String> {
    let client = tokio::net::TcpStream::connect(
        "127.0.0.1:3000"
    )
        .await
        .expect("connection failed");

    let (mut reader, mut writer) = client.into_split();

    let client_read = tokio::spawn(async move {
        tokio::io::copy(&mut reader, &mut tokio::io::stdout()).await.unwrap();
    });

    let client_write = tokio::spawn(async move {
        tokio::io::copy(&mut tokio::io::stdin(), &mut writer).await.unwrap();
    });

    tokio::select! {
        _ = client_read => {}
        _ = client_write => {}
    };

    Ok(())
}

pub async fn server() -> Result<(), String> {
    let listener = tokio::net::TcpListener::bind(
        "127.0.0.1:3000"
    )
        .await
        .expect("failed to bind address");

    let (handel, _) = listener
        .accept()
        .await
        .expect("failed to accept incoming message");

    let (mut reader, mut writer) = handel.into_split();

    let client_read = tokio::spawn(async move {
        tokio::io::copy(&mut reader, &mut tokio::io::stdout()).await.unwrap();
    });

    let client_write = tokio::spawn(async move {
        tokio::io::copy(&mut tokio::io::stdin(), &mut writer).await.unwrap();
    });

    tokio::select! {
        _ = client_read => {}
        _ = client_write => {}
    };

    Ok(())
}