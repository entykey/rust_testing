use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, StreamExt, SinkExt, TryStreamExt};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;

type Tx = futures_channel::mpsc::UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<std::net::SocketAddr, Tx>>>;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Failed to bind");
    let peer_map: PeerMap = Arc::new(Mutex::new(HashMap::new()));

    println!("WebSocket server is running on ws://127.0.0.1:8080");

    while let Ok((stream, addr)) = listener.accept().await {
        let peer_map_clone = peer_map.clone();
        tokio::spawn(handle_connection(stream, addr, peer_map_clone));
    }
}

async fn handle_connection(stream: TcpStream, addr: std::net::SocketAddr, peer_map: PeerMap) {
    if let Ok(ws_stream) = accept_async(stream).await {
        println!("Client connected from: {}", addr);

        let (tx, rx) = unbounded();

        // Insert the sender into the PeerMap
        // let peer_addr = addr;
        peer_map.lock().unwrap().insert(addr, tx);

        let (write, read) = ws_stream.split();

        let broadcast_incomming = read.try_for_each(|msg| {
            println!("Received message from {}: {}", addr, msg.to_text().unwrap());

            let peers = peer_map.lock().unwrap();

            // broadcast message to everyone except the sender.
            let broadcast_recipients = peers
                .iter()
                .filter(|(peer_addr, _)| peer_addr != &&addr)
                .map(|(_, ws_sink)| ws_sink);

            for recp in broadcast_recipients {
                recp.unbounded_send(msg.clone()).unwrap();
            }

            futures_util::future::ok(())
        });

        let receive_from_others = rx.map(Ok).forward(write);

        futures_util::pin_mut!(broadcast_incomming, receive_from_others);
        future::select(broadcast_incomming, receive_from_others).await;

        println!("{} disconnected", &addr);
        peer_map.lock().unwrap().remove(&addr);

    }
}



/*
JS Client:
<!-- Updated HTML and JavaScript code -->
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>WebSocket Client</title>
    <style>
        .self-message {
            color: green;
        }

        .other-message {
            color: red;
        }

        .system-message {
            color: blue;
        }
    </style>
</head>
<body>
    <h1>WebSocket Client</h1>
    <ul id="messageList"></ul>
    <input type="text" id="input" placeholder="Type a message">
    <button onclick="sendMessage()">Send</button>

    <script>
        const messageList = document.getElementById('messageList');
        const input = document.getElementById('input');
        const button = document.querySelector('button');
    
        const socket = new WebSocket('ws://127.0.0.1:8080');
    
        socket.addEventListener('open', (event) => {
            appendMessage('Connected to WebSocket server', 'system-message');
        });
    
        socket.addEventListener('message', (event) => {
            const data = event.data;
            const timestamp = new Date().toLocaleTimeString();
    
            // const parsedData = JSON.parse(data);
            const messageText = `[${timestamp}] Other: ${data}`;
            appendMessage(messageText, 'other-message');

            // if (isValidJSON(data)) {
            //     const parsedData = JSON.parse(data);
            //     const messageText = `[${timestamp}] ${parsedData.isSelf ? 'You' : 'Other'}: ${parsedData.message}`;
            //     addMessage(messageText, 'other-message');
            // }
        });
    
        socket.addEventListener('close', (event) => {
            appendMessage(`[${new Date().toLocaleTimeString()}] Server or Other client connection closed, reason: ${event.reason}`, 'system-message');
        });
    

        function sendMessage() {
            const message = input.value;
            const timestamp = new Date().toLocaleTimeString();

            if (message.trim() !== '') {
                // send to server:
                socket.send(message);

                input.value = '';
                const messageText = `[${timestamp}] You: ${message}`;
                appendMessage(messageText, 'self-message');
            }
        }

        function appendMessage(message, messageType) {
            const listItem = document.createElement('li');
            listItem.textContent = message;
            listItem.className = messageType;
            messageList.appendChild(listItem);
        }
    
        function isValidJSON(jsonString) {
            try {
                JSON.parse(jsonString);
                return true;
            } catch (e) {
                return false;
            }
        }
    </script>
</body>
</html>

*/