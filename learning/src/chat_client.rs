use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::thread;
use std::sync::{Arc, Mutex};

fn receive_messages(stream: Arc<Mutex<TcpStream>>) {
    let mut buffer = [0; 1024];
    loop {
        match stream.lock().unwrap().read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    println!("Server has closed the connection.");
                    break;
                }

                let message = std::str::from_utf8(&buffer[..bytes_read]).unwrap();
                println!("{}", message);
            }
            Err(e) => {
                println!("Error while receiving message: {}", e);
                break;
            }
        }
    }
}

fn send_messages(stream: Arc<Mutex<TcpStream>>) {
    let stdin = io::stdin();
    let mut stdout = io::stdout(); // Create a mutable handle to the stdout

    loop {
        print!("Enter a message: "); // Display prompt
        stdout.flush().unwrap(); // Explicitly flush stdout to display the prompt

        let mut input = String::new();
        stdin.read_line(&mut input).expect("Failed to read line");

        // Exit the loop if the user enters ".exit"
        if input.trim() == ".exit" {
            break;
        }

        stream.lock().unwrap().write_all(input.trim().as_bytes()).expect("Failed to send message");
    }
}




fn main() {
    let stream = TcpStream::connect("127.0.0.1:8080").expect("Failed to connect to server");
    println!("Connected to server.");

    let shared_stream = Arc::new(Mutex::new(stream));
    let receive_thread = thread::spawn({
        let shared_stream = shared_stream.clone();
        move || {
            receive_messages(shared_stream);
        }
    });

    let send_thread = thread::spawn({
        let shared_stream = shared_stream.clone();
        move || {
            send_messages(shared_stream);
        }
    });

    receive_thread.join().expect("Failed to join receive thread");
    send_thread.join().expect("Failed to join send thread");
}