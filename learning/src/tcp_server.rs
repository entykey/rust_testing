// tcp_server.rs
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::SystemTime;

fn handle_client(mut stream: TcpStream) {
    let peer_addr = stream.peer_addr().unwrap();
    println!("Accepted a connection from: {}", peer_addr);

    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    println!("Client {} disconnected", peer_addr);
                    break;
                }

                let message = String::from_utf8_lossy(&buffer[..n]);
                println!("Received from {}, message: {}", peer_addr, message);

                // Echo the received data back to the client
                stream.write_all(&buffer[..n]).unwrap();
            }
            Err(e) => {
                println!("Error reading from client {}: {}", peer_addr, e);
                break;
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");
    println!("Server listening on 127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Spawn a new thread to handle the client connection
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                println!("Error accepting connection: {}", e);
            }
        }
    }
}





// tcp_server.rs (now migrate to tokio async)
/*
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

async fn handle_client(mut stream: TcpStream) {
    let peer_addr = stream.peer_addr().unwrap();
    println!("Accepted a connection from: {}", peer_addr);

    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer).await {
            Ok(n) => {
                if n == 0 {
                    println!("Client {} disconnected", peer_addr);
                    break;
                }

                let message = String::from_utf8_lossy(&buffer[..n]);
                println!("Received from {}, message: {}", peer_addr, message);

                // Echo the received data back to the client
                stream.write_all(&buffer[..n]).await.unwrap();
            }
            Err(e) => {
                println!("Error reading from client {}: {}", peer_addr, e);
                break;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.expect("Failed to bind to address");
    println!("Server listening on 127.0.0.1:8080");

    // loop {
    //     match listener.accept().await {
    //         Ok((stream, _socket_addr)) => {
    //             // Spawn a new thread to handle the client connection
    //             // thread::spawn(|| handle_client(stream));
    //             tokio::spawn(async {
    //                 handle_client(stream).await;
    //             });
    //         }
    //         Err(e) => {
    //             println!("Error accepting connection: {}", e);
    //         }
    //     }
    // }

    while let Ok((stream, _socket_addr)) = listener.accept().await {
        // Spawn a new thread to handle the client connection
        tokio::spawn(async {
            handle_client(stream).await;
        });
    }
}
*/