use crate::common::read_write_exec;

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

pub async fn client(host: &str, port: u16) -> Result<(), String> {
    let addr = format!("{}:{}", host, port);

    let client = tokio::net::TcpStream::connect(&addr)
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

pub async fn server(host: &str, port: u16) -> Result<(), String> {
    let addr = format!("{}:{}", host, port);

    let listener = tokio::net::TcpListener::bind(&addr)
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

pub async fn server_exec(host: &str, port: u16, cmd: &str) -> Result<(), String> {
    let addr = format!("{}:{}", host, port);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("failed to bind address");

    let (handel, _) = listener
        .accept()
        .await
        .expect("failed to accept incoming message");

    let (reader, writer) = handel.into_split();

    read_write_exec(reader, writer, cmd).await;

    Ok(())
}