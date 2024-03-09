use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

#[tokio::main]
async fn main() {
    let addr = "localhost:8080";
    let mut stream = TcpStream::connect(addr).await.unwrap();

    let (reader, mut writer) = stream.split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    // Spawn a task to read messages from the server
    tokio::spawn(async move {
        loop {
            match reader.read_line(&mut line).await {
                Ok(0) => break, // Connection closed by the server
                Ok(_) => println!("Server: {}", line),
                Err(_) => break, // Error reading from server
            }
            line.clear();
        }
    });

    // Loop to read input from the user and send to the server
    loop {
        line.clear();
        if let Ok(n) = std::io::stdin().read_line(&mut line) {
            if n == 0 {
                break; // Exit on Ctrl-D (Unix-like) or Ctrl-Z (Windows)
            }
        }

        // Send the user input to the server
        if let Err(_) = writer.write_all(line.as_bytes()).await {
            break; // Error writing to server
        }
        writer.flush().await.unwrap();
    }
}