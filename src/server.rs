use std::{net::TcpListener, io::{Write, Read, stdout}};

pub fn run(host: &str, port: usize) -> Result<(), String> {
    let addr = format!("{}:{}", host, port);

    let listener = TcpListener::bind(addr).expect("listener failed");

    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => {
                println!("Connection accepted");
                let mut buffer = [0; 128];
                let mut read_count = 0;
                while read_count == 0 {
                    read_count = s.read(&mut buffer).expect("Couldn't read buffer");
                    println!("received bytes {}", read_count);
                }

                stdout().write(&buffer[0..read_count]).expect("Writing to stdout failed");
                stdout().flush().unwrap();
            },
            Err(e) => {
                println!("Error while accepting incoming connection - {}", e);
            }
        }
    }

    Ok(())
}