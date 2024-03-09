// tcp_client.rs
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {

    // Connect to the server
    match TcpStream::connect("127.0.0.1:8080") {
        Ok(mut stream) => {
            println!("Connected to server");

            let mut count: i32 = 0;

            // Send data to the server

            // without this loop, the client disconnect after send & receive
            loop {

                let message: String = format!("Hello, server! {count}");

                // Write data to server/socket
                match stream.write_all(message.as_bytes()) {
                    Ok(_) => {
                        println!("written bytes data: {:?}", message.as_bytes());
                        count += 1;
                    }
                    Err(err) => {
                        println!("Error writing socket: {}", err);
                    }
                }

                // Receive the echo response from the server
                let mut buffer: [u8; 1024] = [0; 1024];
                match stream.read(&mut buffer) {
                    Ok(n) => {
                        let response: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer[..n]);
                        println!("Received: {}", response);
                    }
                    Err(e) => {
                        println!("Error reading from server: {}", e);
                    }
                }

                // add delay
                // std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}



// tcp_client.rs (now migrate to tokio async)
/*
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {

    // Connect to the server
    match TcpStream::connect("127.0.0.1:8080").await {
        Ok(mut stream) => {
            println!("Connected to server");

            let mut count: i32 = 0;

            // Send data to the server

            // without this loop, the client disconnect after send & receive
            loop {

                let message: String = format!("Hello, server! {count}");

                // Write data to server/socket
                match stream.write_all(message.as_bytes()).await {
                    Ok(_) => {
                        println!("written bytes data: {:?}", message.as_bytes());
                        count += 1;
                    }
                    Err(err) => {
                        println!("Error writing socket: {}", err);
                    }
                }

                // Receive the echo response from the server
                let mut buffer: [u8; 1024] = [0; 1024];
                match stream.read(&mut buffer).await {
                    Ok(n) => {
                        let response: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer[..n]);
                        println!("Received: {}", response);
                    }
                    Err(e) => {
                        println!("Error reading from server: {}", e);
                    }
                }

                // add delay
                // tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}
*/