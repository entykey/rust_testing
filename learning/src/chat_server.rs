use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::{Arc, Mutex};

fn handle_client(mut stream: TcpStream, clients: Arc<Mutex<Vec<TcpStream>>>) {
    let peer_addr = stream.peer_addr().unwrap();
    println!("New client connected: {}", peer_addr);

    let welcome_msg = format!("Welcome to the chat, {}!\n", peer_addr);
    stream.write_all(welcome_msg.as_bytes()).unwrap();

    let mut buffer = [0; 1024];
    loop {
        let bytes_read = match stream.read(&mut buffer) {
            Ok(bytes) => bytes,
            Err(_) => break,
        };

        if bytes_read == 0 {
            break;
        }

        let message = std::str::from_utf8(&buffer[..bytes_read]).unwrap();
        let broadcast_msg = format!("{}: {}", peer_addr, message);
        broadcast_message(broadcast_msg.as_bytes(), &clients);
    }

    println!("Client {} disconnected.", peer_addr);

    // Remove the disconnected client from the list
    clients.lock().unwrap().retain(|client| client.peer_addr().unwrap() != peer_addr);
}

fn broadcast_message(message: &[u8], clients: &Arc<Mutex<Vec<TcpStream>>>) {
    let mut disconnected_clients = Vec::new();

    for mut stream in clients.lock().unwrap().iter() {
        if let Err(_) = stream.write_all(message) {
            // The write failed, the client is disconnected
            disconnected_clients.push(stream.peer_addr().unwrap());
        }
    }

    // Remove the disconnected clients from the list
    clients.lock().unwrap().retain(|client| !disconnected_clients.contains(&client.peer_addr().unwrap()));
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");
    let clients: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(vec![]));

    // Get the server's socket address
    let server_addr = listener.local_addr().expect("Failed to get server address");
    println!("Server is running on {}:{}", server_addr.ip(), server_addr.port());

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let clients_clone = Arc::clone(&clients);
                clients.lock().unwrap().push(stream.try_clone().expect("Failed to clone client"));
                thread::spawn(move || {
                    handle_client(stream, clients_clone);
                });
            }
            Err(e) => {
                println!("Error while accepting client: {}", e);
            }
        }
    }
}
