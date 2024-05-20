/*
// 3 errors: no method named `lock` found for struct `Arc<HashMap<usize, SplitSink<WebSocketStream<tokio::net::TcpStream>, Message>>>` in the current scope
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use rand::Rng;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Task {
    id: usize,
    text: String,
    status: String,
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await.expect("Failed to bind");
    let clients: Arc<HashMap<usize, SplitSink<tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>, Message>>> = Arc::new(HashMap::new());
    let tasks: Arc<HashMap<usize, Task>> = Arc::new(HashMap::new());
    let task_id = Arc::new(tokio::sync::Mutex::new(0));

    loop {
        let (stream, _) = listener.accept().await.expect("Failed to accept");
        let clients = Arc::clone(&clients);
        let tasks = Arc::clone(&tasks);
        let task_id = Arc::clone(&task_id);

        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await.expect("Error during WebSocket handshake");
            let (sink, mut stream) = ws_stream.split();
            let mut client_id = 0;

            {
                let mut clients = clients.lock().await;
                client_id = clients.len() + 1;
                clients.insert(client_id, sink);
            }

            // Send current tasks to the newly connected client
            for task in tasks.values() {
                if let Ok(serialized_task) = serde_json::to_string(&task) {
                    if let Err(e) = clients[&client_id].send(Message::Text(serialized_task)).await {
                        eprintln!("Error sending message to client: {:?}", e);
                    }
                }
            }

            while let Some(msg) = stream.next().await {
                match msg {
                    Ok(msg) => {
                        if let Message::Text(text) = msg {
                            if let Ok(received_task) = serde_json::from_str::<Task>(&text) {
                                let mut tasks = tasks.lock().await;
                                match received_task.status.as_str() {
                                    "create" => {
                                        let mut task_id = task_id.lock().await;
                                        *task_id += 1;
                                        received_task.id = *task_id;
                                        tasks.insert(received_task.id, received_task.clone());
                                    }
                                    "update" => {
                                        if tasks.contains_key(&received_task.id) {
                                            tasks.insert(received_task.id, received_task.clone());
                                        }
                                    }
                                    "delete" => {
                                        tasks.remove(&received_task.id);
                                    }
                                    _ => {}
                                }
                                if let Ok(serialized_task) = serde_json::to_string(&received_task) {
                                    for client in clients.values() {
                                        if let Err(e) = client.send(Message::Text(serialized_task.clone())).await {
                                            eprintln!("Error sending message to client: {:?}", e);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error receiving message: {:?}", e);
                        break;
                    }
                }
            }

            let mut clients = clients.lock().await;
            clients.remove(&client_id);
        });
    }
}
*/




/*
// 1 error remaining
// explanatory: Apologies for the oversight. It seems I mistakenly used lock() on Arc instances directly, which is incorrect. We should first dereference Arc to access the HashMap and then use lock() on the resulting Mutex. Let me correct that for you:
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use rand::Rng;
use tokio::sync::Mutex;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Task {
    id: usize,
    text: String,
    status: String,
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await.expect("Failed to bind");
    let clients: Arc<Mutex<HashMap<usize, SplitSink<tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>, Message>>>> = Arc::new(Mutex::new(HashMap::new()));
    let tasks: Arc<Mutex<HashMap<usize, Task>>> = Arc::new(Mutex::new(HashMap::new()));
    let task_id = Arc::new(Mutex::new(0));

    loop {
        let (stream, _) = listener.accept().await.expect("Failed to accept");
        let clients: Arc<Mutex<HashMap<usize, SplitSink<tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>, Message>>>> = Arc::clone(&clients);
        let tasks = Arc::clone(&tasks);
        let task_id = Arc::clone(&task_id);

        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await.expect("Error during WebSocket handshake");
            let (sink, mut stream) = ws_stream.split();
            let mut client_id = 0;

            {
                let mut clients: tokio::sync::MutexGuard<HashMap<usize, SplitSink<tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>, Message>>> = clients.lock().await;
                client_id = clients.len() + 1;
                clients.insert(client_id, sink);
            }

            // Send current tasks to the newly connected client
            for task in tasks.lock().await.values() {
                if let Ok(serialized_task) = serde_json::to_string(&task) {
                    if let Err(e) = clients.get_mut().lock().await[&client_id].send(Message::Text(serialized_task)).await {
                        eprintln!("Error sending message to client: {:?}", e);
                    }
                }
            }

            while let Some(msg) = stream.next().await {
                match msg {
                    Ok(msg) => {
                        if let Message::Text(text) = msg {
                            if let Ok(mut received_task) = serde_json::from_str::<Task>(&text) {
                                let mut tasks = tasks.lock().await;
                                match received_task.status.as_str() {
                                    "create" => {
                                        let mut task_id = task_id.lock().await;
                                        *task_id += 1;
                                        received_task.id = *task_id;
                                        tasks.insert(received_task.id, received_task.clone());
                                    }
                                    "update" => {
                                        if tasks.contains_key(&received_task.id) {
                                            tasks.insert(received_task.id, received_task.clone());
                                        }
                                    }
                                    "delete" => {
                                        tasks.remove(&received_task.id);
                                    }
                                    _ => {}
                                }
                                if let Ok(serialized_task) = serde_json::to_string(&received_task) {
                                    for client in clients.lock().await.values() {
                                        if let Err(e) = client.send(Message::Text(serialized_task.clone())).await {
                                            eprintln!("Error sending message to client: {:?}", e);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error receiving message: {:?}", e);
                        break;
                    }
                }
            }

            let mut clients = clients.lock().await;
            clients.remove(&client_id);
        });
    }
}
*/




/*
// Explanatory: It seems the issue arises because lock() is being called on a mutable reference to HashMap, which doesn't have a lock() method. Instead, you should call lock() on the Mutex itself. Let's correct that now
// Implement STATUS: compiled for now, allowed mutiple clients
// to connect, realtime collaborative text editing worked,
// but the todo tasks are not syncing with each clients,
// it's different list in each connected client,
// and the delete method also does not work.
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use rand::Rng;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::net::TcpListener;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Task {
    id: usize,
    text: String,
    status: String,
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await.expect("Failed to bind");
    let clients: Arc<Mutex<HashMap<usize, SplitSink<tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>, Message>>>> = Arc::new(Mutex::new(HashMap::new()));
    let tasks: Arc<Mutex<HashMap<usize, Task>>> = Arc::new(Mutex::new(HashMap::new()));
    let task_id = Arc::new(Mutex::new(0));

    loop {
        let (stream, _) = listener.accept().await.expect("Failed to accept");
        let clients = Arc::clone(&clients);
        let tasks = Arc::clone(&tasks);
        let task_id = Arc::clone(&task_id);

        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await.expect("Error during WebSocket handshake");
            let (sink, mut stream) = ws_stream.split();
            let mut client_id = 0;

            {
                let mut clients = clients.lock().await;
                client_id = clients.len() + 1;
                clients.insert(client_id, sink);
            }

            // Send current tasks to the newly connected client
            for task in tasks.lock().await.values() {
                if let Ok(serialized_task) = serde_json::to_string(&task) {
                    if let Some(mut client) = clients.lock().await.get_mut(&client_id) {
                        if let Err(e) = client.send(Message::Text(serialized_task)).await {
                            eprintln!("Error sending message to client: {:?}", e);
                        }
                    }
                }
            }

            while let Some(msg) = stream.next().await {
                match msg {
                    Ok(msg) => {
                        if let Message::Text(text) = msg {
                            if let Ok(mut received_task) = serde_json::from_str::<Task>(&text) {
                                let mut tasks = tasks.lock().await;
                                match received_task.status.as_str() {
                                    "create" => {
                                        let mut task_id = task_id.lock().await;
                                        *task_id += 1;
                                        received_task.id = *task_id;
                                        tasks.insert(received_task.id, received_task.clone());
                                    }
                                    "update" => {
                                        if tasks.contains_key(&received_task.id) {
                                            tasks.insert(received_task.id, received_task.clone());
                                        }
                                    }
                                    "delete" => {
                                        tasks.remove(&received_task.id);
                                    }
                                    _ => {}
                                }
                                if let Ok(serialized_task) = serde_json::to_string(&received_task) {
                                    for client in clients.lock().await.values_mut() {
                                        if let Err(e) = client.send(Message::Text(serialized_task.clone())).await {
                                            eprintln!("Error sending message to client: {:?}", e);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error receiving message: {:?}", e);
                        break;
                    }
                }
            }

            let mut clients = clients.lock().await;
            clients.remove(&client_id);
        });
    }
}
*/





/*
STATUS: 1 err: error[E0599]: no method named `lock` found for struct `tokio::sync::MutexGuard<'_, HashMap<usize, Task>>` in the current scope
    --> learning/src/main.rs:6346:55
     |
6346 | ...                   let mut tasks = tasks.lock().await;
     |                                             ^^^^ private field, not a method

For more information about this error, try `rustc --explain E0599`
*/
/*
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use rand::Rng;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};
use tokio::net::TcpListener;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Task {
    id: usize,
    text: String,
    status: String,
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await.expect("Failed to bind");
    let clients: Arc<Mutex<HashMap<usize, SplitSink<tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>, Message>>>> = Arc::new(Mutex::new(HashMap::new()));
    let tasks: Arc<Mutex<HashMap<usize, Task>>> = Arc::new(Mutex::new(HashMap::new()));
    let task_id = Arc::new(Mutex::new(0));
    let (sender, _) = broadcast::channel(10); // Create a broadcast channel

    loop {
        let (stream, _) = listener.accept().await.expect("Failed to accept");
        let clients = Arc::clone(&clients);
        let tasks = Arc::clone(&tasks);
        let task_id = Arc::clone(&task_id);
        let sender = sender.clone(); // Clone the sender for each new connection

        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await.expect("Error during WebSocket handshake");
            let (sink, mut stream) = ws_stream.split();
            let mut client_id = 0;

            {
                let mut clients = clients.lock().await;
                client_id = clients.len() + 1;
                clients.insert(client_id, sink);
            }

            // Send current tasks to the newly connected client
            let tasks = tasks.lock().await;
            for task in tasks.values() {
                if let Ok(serialized_task) = serde_json::to_string(&task) {
                    if let Some(mut client) = clients.lock().await.get_mut(&client_id) {
                        if let Err(e) = client.send(Message::Text(serialized_task)).await {
                            eprintln!("Error sending message to client: {:?}", e);
                        }
                    }
                }
            }

            while let Some(msg) = stream.next().await {
                match msg {
                    Ok(msg) => {
                        if let Message::Text(text) = msg {
                            if let Ok(mut received_task) = serde_json::from_str::<Task>(&text) {
                                let mut tasks = tasks.lock().await;
                                match received_task.status.as_str() {
                                    "create" => {
                                        let mut task_id = task_id.lock().await;
                                        *task_id += 1;
                                        received_task.id = *task_id;
                                        tasks.insert(received_task.id, received_task.clone());
                                    }
                                    "update" => {
                                        if let Some(existing_task) = tasks.get_mut(&received_task.id) {
                                            existing_task.text = received_task.text.clone();
                                            existing_task.status = received_task.status.clone();
                                        }
                                    }
                                    "delete" => {
                                        tasks.remove(&received_task.id);
                                    }
                                    _ => {}
                                }
                                
                                // Broadcast the updated task list to all clients
                                let _ = sender.send(received_task.clone());
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error receiving message: {:?}", e);
                        break;
                    }
                }
            }

            let mut clients = clients.lock().await;
            clients.remove(&client_id);
        });

        // Listen for task updates and broadcast them to all clients
        let receiver = sender.subscribe();
        tokio::spawn(async move {
            while let Ok(updated_task) = receiver.recv().await {
                let tasks = tasks.lock().await;
                let serialized_task = serde_json::to_string(&updated_task).unwrap();
                for client in clients.lock().await.values_mut() {
                    if let Err(e) = client.send(Message::Text(serialized_task.clone())).await {
                        eprintln!("Error broadcasting task update to client: {:?}", e);
                    }
                }
            }
        });
    }
}
*/