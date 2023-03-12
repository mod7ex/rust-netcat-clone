use tokio::{
    io::{
        stdout,
        stdin,
        AsyncReadExt,
        AsyncWriteExt
    },
    net::UdpSocket,
    select
};

pub async fn udp_connect(host: &str, port: u16, listen_port: u16) -> Result<(), String> {
    let socket = UdpSocket::bind(
        format!("0.0.0.0:{}", listen_port)
    ).await.expect("failed to bind port");

    socket.connect(
        format!("{}:{}", host, port)
    ).await.expect("failed to connect");

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

pub async fn udp_serve(host: &str, port: u16) -> Result<(), String> {
    let socket = UdpSocket::bind(
        format!("{}:{}", host, port)
    ).await.expect("failed to bind port");

    let mut stdin_buffer = [0; 265];
    let mut network_in_buffer = [0; 256];

    let mut stdin = stdin();

    let mut is_connected = false;

    loop {
        select! {
            res = socket.recv_from(&mut network_in_buffer) => {
                if let Ok((amount, remote_addr)) = res {
                    if !is_connected {
                        socket.connect(remote_addr).await.unwrap();
                        is_connected = true;
                    }

                    stdout()
                        .write(&network_in_buffer[0..amount])
                        .await
                        .expect("failed to write to stdout");
                } else {
                    res.unwrap();
                }
            }

            res = stdin.read(&mut stdin_buffer) => {
                if let Ok(amount) = res {
                    if is_connected {
                        socket
                            .send(&stdin_buffer[0..amount])
                            .await
                            .expect("failed to send data");
                    }
                } else {
                    res.unwrap();
                }
            }
        }
    }
}
