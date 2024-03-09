
// Cursor tracking (server.rs)
extern crate ws;
extern crate env_logger;

use chrono::{Local, DateTime};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use ws::{listen, CloseCode, Handler, Handshake, Message, Sender, Result};

#[derive(Debug, Clone)]
struct Client {
    username: Option<String>,
    sender: Sender,
}

fn main() {
    env_logger::init();

    // Shared state for connected clients and cursor positions
    let connected_clients: Arc<Mutex<HashMap<Sender, Client>>> = Arc::new(Mutex::new(HashMap::new()));

    // WebSocket server
    listen("127.0.0.1:3012", move |out| {
        let connected_clients: Arc<Mutex<HashMap<Sender, Client>>> = connected_clients.clone();

        Server {
            out,
            connected_clients: connected_clients.clone(),
        }
    }).unwrap();
}

struct Server {
    out: Sender,
    connected_clients: Arc<Mutex<HashMap<Sender, Client>>>,
}

impl Handler for Server {
    fn on_open(&mut self, handshake: Handshake) -> Result<()> {
        let client = Client {
            username: None,
            sender: self.out.clone(),
        };

        let hashed_key: String = handshake.request.hashed_key().unwrap();
        let remote_addr: String = handshake.remote_addr()?.ok_or(ws::Error::new(ws::ErrorKind::Internal, "No remote address"))?;
        let connected_clients: Arc<Mutex<HashMap<Sender, Client>>> = self.connected_clients.clone();

        self.connected_clients.lock().unwrap().insert(self.out.clone(), client);
        println!("[{}] WebSocket connection opened from ip: {} hashedkey: {}", str_datetime(), remote_addr, hashed_key);
        println!("Connected clients: {:?}", connected_clients);
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        match msg {
            Message::Text(text) => {
                if let Ok(data) = serde_json::from_str::<HashMap<String, serde_json::Value>>(&text) {
                    if let (Some(username), Some(x), Some(y)) = (
                        data.get("username").and_then(|u| u.as_str()),
                        data.get("x").and_then(|x| x.as_i64()),
                        data.get("y").and_then(|y| y.as_i64()),
                    ) {
                        let mut connected_clients: std::sync::MutexGuard<'_, HashMap<Sender, Client>> = self.connected_clients.lock().unwrap();

                        if let Some(client) = connected_clients.get_mut(&self.out) {
                            client.username = Some(username.to_string());
                            println!(
                                "Received message: {{\"username\":\"{}\", \"x\":{}, \"y\":{}}}",
                                username, x, y
                            );
                        }
                    }
                }
            }
            _ => (),
        }

        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        let mut connected_clients: std::sync::MutexGuard<'_, HashMap<Sender, Client>> = self.connected_clients.lock().unwrap();

        if let Some(client) = connected_clients.remove(&self.out) {
            if let Some(username) = &client.username {
                println!("WebSocket closed for user {} ({:?}): {}", username, code, reason);
            }
        }
    }
}
// 日付の文字列を取得
fn str_datetime() -> String {
    // メッセージに日付を付与
    let local_datetime: DateTime<Local> = Local::now();
    let formatted_local_datetime: String = local_datetime.format("%Y-%m-%d %T").to_string();
    return formatted_local_datetime;
}

/*
// output:
//   [2023-12-09 19:25:34] WebSocket connection opened from ip: 127.0.0.1 hashedkey: tJEZN20Lf4cHxBTejtYHrk9VItk=
//   Received message: {"username":"tuanhayho", "x":192, "y":128}
//   Connected clients: Mutex { data: {Sender { token: Token(0), channel: mio::channel::SyncSender<Command>, connection_id: 0 }: Client { username: None, sender: Sender { token: Token(0), channel: mio::channel::SyncSender<Command>, connection_id: 0 } }}, poisoned: false, .. }
*/



/* 
JS client:
<!-- ws-rs cursor tracking client.html -->
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Cursor Tracker</title>

    <style>
        body {
            font-family: Arial, sans-serif;
        }

        .status-box {
            padding: 10px;
            margin-bottom: 10px;
            display: none;
        }

        .online {
            background-color: #d4edda;
            border-color: #c3e6cb;
            color: #155724;
        }

        .offline {
            background-color: #f8d7da;
            border-color: #f5c6cb;
            color: #721c24;
        }
    </style>
</head>
<body>
    <div class="status-box" id="statusBox">
        <strong>Status:</strong> <span id="statusText"></span>
    </div>

    <h2>Enter your username and click Connect</h2>
    <input type="text" id="usernameInput" placeholder="Enter your username">
    <button onclick="connect()">Connect</button>

    <script>
        let socket;
        const statusBox = document.getElementById('statusBox');
        const statusText = document.getElementById('statusText');
        const usernameInput = document.getElementById('usernameInput');

        function connect() {
            const username = usernameInput.value.trim();

            if (!username) {
                alert('Please enter a valid username.');
                return;
            }

            socket = new WebSocket("ws://localhost:3012");
            socket.addEventListener("open", (event) => {
                console.log("Connected to websocket server!");
                updateStatus('Online', 'online');
                // Send the username to the server
                socket.send(JSON.stringify({ username }));
            });

            socket.addEventListener("close", (event) => {
                console.log("WebSocket connection closed:", event);
                updateStatus('Server Disconnected', 'offline');
                alert("WebSocket connection closed:", event);
            });

            // Track cursor position and send updates continuously
            document.addEventListener("mousemove", (event) => {
                try {
                    // Check if the socket is open before sending data
                    if (socket.readyState === WebSocket.OPEN) {
                        const position = { username, x: event.clientX, y: event.clientY };
                        socket.send(JSON.stringify(position));
                        // console.log("sent to ws server:", position);
                    } else {
                        console.warn("WebSocket is not open. Cannot send data.");
                    }
                } catch (error) {
                    console.error("Error sending WebSocket message:", error);
                }
            });
        }

        function updateStatus(text, className) {
            statusText.textContent = text;
            statusBox.className = 'status-box ' + className;
            statusBox.style.display = 'block';
        }
    </script>
</body>
</html>
 */