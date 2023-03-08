use std::{net::TcpStream, io::Write};

pub fn run(host: &str, port: usize) -> Result<(), String>{
    let addr = format!("{}:{}", host, port);

    let mut client = TcpStream::connect(addr).expect("connection failed");

    client.write("hello, TCP".as_bytes()).expect("msg");

    Ok(())
}