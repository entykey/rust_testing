/*
//! An UDP echo server that just sends back everything that it receives.
//!
//! If you're on Unix you can test this out by in one terminal executing:
//!
//!     cargo run --example echo-udp
//!
//! and in another terminal you can run:
//!
//!     cargo run --example connect -- --udp 127.0.0.1:8080
//!
//! Each line you type in to the `nc` terminal should be echo'd back to you!

#![warn(rust_2018_idioms)]

use std::error::Error;
use std::net::SocketAddr;
use std::{env, io};
use tokio::net::UdpSocket;

struct Server {
    socket: UdpSocket,
    buf: Vec<u8>,
    to_send: Option<(usize, SocketAddr)>,
}

impl Server {
    async fn run(self) -> Result<(), io::Error> {
        let Server {
            socket,
            mut buf,
            mut to_send,
        } = self;

        loop {
            // First we check to see if there's a message we need to echo back.
            // If so then we try to send it back to the original source, waiting
            // until it's writable and we're able to do so.
            if let Some((size, peer)) = to_send {
                let amt = socket.send_to(&buf[..size], &peer).await?;

                println!("Echoed {}/{} bytes to {}", amt, size, peer);
            }

            // If we're here then `to_send` is `None`, so we take a look for the
            // next message we're going to echo back.
            to_send = Some(socket.recv_from(&mut buf).await?);
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let socket = UdpSocket::bind(&addr).await?;
    println!("Listening on: {}", socket.local_addr()?);

    let server = Server {
        socket,
        buf: vec![0; 1024],
        to_send: None,
    };

    // This starts the server task.
    server.run().await?;

    Ok(())
}
*/






/*
//! A UDP client that just sends everything it gets via `stdio` in a single datagram, and then
//! waits for a reply.
//!
//! For the reasons of simplicity data from `stdio` is read until `EOF` in a blocking manner.
//!
//! You can test this out by running an echo server:
//!
//! ```
//!     $ cargo run --example echo-udp -- 127.0.0.1:8080
//! ```
//!
//! and running the client in another terminal:
//!
//! ```
//!     $ cargo run --example udp-client
//! ```
//!
//! You can optionally provide any custom endpoint address for the client:
//!
//! ```
//!     $ cargo run --example udp-client -- 127.0.0.1:8080
//! ```
//!
//! Don't forget to pass `EOF` to the standard input of the client!
//!
//! Please mind that since the UDP protocol doesn't have any capabilities to detect a broken
//! connection the server needs to be run first, otherwise the client will block forever.

#![warn(rust_2018_idioms)]

use std::env;
use std::error::Error;
use std::io::{stdin, Read};
use std::net::SocketAddr;
use tokio::net::UdpSocket;

fn get_stdin_data() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut buf = Vec::new();
    stdin().read_to_end(&mut buf)?;
    Ok(buf)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let remote_addr: SocketAddr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".into())
        .parse()?;

    // We use port 0 to let the operating system allocate an available port for us.
    let local_addr: SocketAddr = if remote_addr.is_ipv4() {
        "0.0.0.0:0"
    } else {
        "[::]:0"
    }
    .parse()?;

    println!("binding addr: {}", &remote_addr);
    let socket = UdpSocket::bind(local_addr).await?;
    const MAX_DATAGRAM_SIZE: usize = 65_507;
    socket.connect(&remote_addr).await?;
    let data = get_stdin_data()?;
    socket.send(&data).await?;
    let mut data = vec![0u8; MAX_DATAGRAM_SIZE];
    let len = socket.recv(&mut data).await?;
    println!(
        "Received {} bytes:\n{}",
        len,
        String::from_utf8_lossy(&data[..len])
    );

    Ok(())
}
*/





/*
// WARN: not completed yet
// UDP socket programming (documented, still researching)
// use tokio::net::UdpSocket;
use std::{io, net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket}};

#[tokio::main]
async fn main() {
    /*
    let sock_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);

    let sock = UdpSocket::bind(sock_addr).await
        .expect("couldn't bind to address");

    println!("UdpSocket listenning on {}", sock_addr);

    let remote_addr = "127.0.0.1:8080";
    
    tokio::time::sleep(tokio::time::Duration::from_millis(400)).await;

    sock.connect(remote_addr).await
        .expect("couldn't connect to address");

    let mut buf = [0; 1024];
    loop {
        let len = sock.recv(&mut buf).await?;
        println!("{:?} bytes received from {:?}", len, remote_addr);

        let len = sock.send(&buf[..len]).await?;
        println!("{:?} bytes sent", len);
    }
    */




    let sock_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);

    let socket = UdpSocket::bind(sock_addr)
        .expect("couldn't bind to address");
    
    socket.set_nonblocking(true).unwrap();

    // clonning the socket so that we have one for sending, one for receiving
    let socket_clone = socket.try_clone().expect("couldn't clone the socket");

    socket.connect(sock_addr)
        .expect("couldn't connect to address");

    // set the value of the SO_BROADCAST option of the socket
    // socket.set_broadcast(false)
    //     .expect("set_broadcast call failed");

    // get the value of the SO_BROADCAST option of the socket
    println!("socket.broadcast: {}", socket.broadcast().unwrap());

    println!("socket.local_addr(): {}", socket.local_addr().unwrap());

    // sending task (tokio::spawn or async_std::task::spawn works)
    std::thread::spawn( move || {

        // let main_time: tokio::time::Instant = tokio::time::Instant::now();


        // if counter has type u8, it will only send 255 times.
        let mut i = 0;
        
        for i in i..= 100001 { // loop
            // Sends data on the socket to the remote address to which it is connected. 
            // On success, returns the number of bytes written.
            // Note that the operating system may refuse buffers larger than 65507. However, partial writes are not possible until buffer sizes above i32::MAX.
            
            // socket_clone.send(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11])
            //     .expect("couldn't send message");

            let message_string = format!("Hello Tứng hay ho 👨🏻‍💻 {i}");
            let decoded_message_bytes = message_string.as_bytes();
            
            socket_clone.send(&decoded_message_bytes)
                .expect("couldn't send message");

            // i+= 1;

            // delay each sending (fast means possible package lost, not all sent package received properly)
            // tokio::time::sleep(tokio::time::Duration::from_millis(20)).await;
        }

        // End of main
        // let duration: std::time::Duration = main_time.elapsed();
        // let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
        // println!("\n⌛️ Execution time: {:?} ({:?} ms)", duration, elapsed_ms);
    });
    
    // receiving task
    loop {
        // Returns the socket address of the remote peer this socket was connected to.
        // match socket.peer_addr() {
        //     Ok(peer_addr) => println!("socket.peer_addr(): {}", peer_addr),

        //     // directly get err kind:  socket.peer_addr().unwrap_err().kind()
        //     Err(err) => println!("Error reading peer_addr, kind: {}", err.kind())
        // }

        // maximum buffering size
        let mut buf: [u8; 100] = [0; 100];

        let (num_bytes_read, _) = loop {
            match socket.recv_from(&mut buf) {
                Ok(n) => break n,
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // wait until network socket is ready, typically implemented
                    // via platform-specific APIs such as epoll or IOCP
                    // wait_for_fd();
                    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                }
                // Err(e) => panic!("encountered IO error: {e}"),
                Err(e) => println!("encountered IO error: {e}"),
            }
        };
        // println!("received bytes: {:?}", &buf[..num_bytes_read]);
        // let decoded_message = match std::str::from_utf8(&buf) {
        //     Ok(v) => v,
        //     Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        // };

        // https://stackoverflow.com/questions/19076719/how-do-i-convert-a-vector-of-bytes-u8-to-a-string
        // use `String::from_utf8_lossy(), It turns invalid UTF-8 bytes into � and so no error handling is required`
        // let decoded_message = String::from_utf8_lossy(&buf);
        // println!("decoded msg: {}", decoded_message);
    }

}
*/




// C# client rust server
/*
use std::{io, net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket}};

#[tokio::main]
async fn main() {
    let sock_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);

    let socket = UdpSocket::bind(sock_addr).expect("couldn't bind to address");
    
    socket.set_nonblocking(true).unwrap();

    println!("UdpSocket listenning on {}", sock_addr);

    let mut buf = [0; 100];

    loop {
        match socket.recv_from(&mut buf) {
            Ok((num_bytes_read, src_addr)) => {
                let received_message = std::str::from_utf8(&buf[..num_bytes_read]).unwrap();
                println!("Received from {}: {}", src_addr, received_message);

                // Send "hello client" back to the client
                let response_message = "hello client";
                socket.send_to(response_message.as_bytes(), src_addr).expect("send_to failed");
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // Do nothing if there's no data to read
            }
            Err(e) => println!("encountered IO error: {e}"),
        }
    }
}
*/




// working code (documented)
// Rust UDP server for C# client (try to seperate thread)
/*
use std::{io, net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket}, sync::{Arc, Mutex}};

#[tokio::main]
async fn main() {

    // let list: Vec<String> = vec![];
    // let client_list = Arc::new(Mutex::new(list));
    let client_list: Arc<Mutex<Vec<SocketAddr>>> = Arc::new(Mutex::new(Vec::new()));

    // Solution: Clone the Arc before you capture it in the closure. ref: https://users.rust-lang.org/t/using-arc-mutex-t-compiler-tells-me-i-am-using-a-moved-value/79628
    let client_list_clone = Arc::clone(&client_list);

    let sock_addr: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);

    let socket: UdpSocket = UdpSocket::bind(sock_addr).expect("couldn't bind to address");
    
    socket.set_nonblocking(true).unwrap();

    // clonning the socket so that we have one for sending, one for receiving
    let socket_clone = socket.try_clone().expect("couldn't clone the socket");

    println!("UdpSocket opened on {}, listenning for incomming messages", sock_addr);

    let mut buf: [u8; 100] = [0; 100];

    
    let _server_seperate_broadcast_channel = tokio::spawn(async move {
        loop {

            // This send task will send forever (even if no receiver is online) 
            // (since we dont know if they're still on the socket)
            // Send "server seperate broadcast" to the all participants (including server & clients)
            let message = "server seperate broadcast";
            // socket_clone.send_to(message.as_bytes(), sock_addr).expect("send_to failed");
            
            // create a copy of `client_list_clone` for this spawned task
            // if just lock the main's client_list_clone without making a copy, the next spawned task will not be able to use it
            let client_list_clone = Arc::clone(&client_list);
            let mut client_list_guard = client_list_clone.lock().unwrap();
            for &client_addr in client_list_guard.iter() {
                socket_clone
                    .send_to(message.as_bytes(), client_addr)
                        .expect("send_to failed");
                
                println!("Sent broadcast message to {}", client_addr);
            }

            // You wont be able to use them inside a normal spawned task because those could be moved across threads at any .await.
            // the below line cause the following error: https://users.rust-lang.org/t/future-is-not-send-inside-tokio-spawn/64898/2
            tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
        }
        
    });
    

    let _server_main_loop = tokio::spawn(async move {
        loop {
            match socket.recv_from(&mut buf) {
                Ok((num_bytes_read, sender_addr)) => {

                    // here we can obtain the SocketAddr of sender (client) and send back message to him.
                    // let client_list_clone = Arc::clone(&client_list);
                    let mut client_list_guard = client_list_clone.lock().unwrap();
                    client_list_guard.push(sender_addr);

                    // Immediately respond to sender:
                    let received_message = std::str::from_utf8(&buf[..num_bytes_read]).unwrap();
                    println!("Received from {}: {}", sender_addr, received_message);   // eg. Received from 127.0.0.1:53209: Hi server, 206
    
                    // Send "hello client" back to the client
                    let response_message = "hello client";
                    socket.send_to(response_message.as_bytes(), sender_addr).expect("send_to failed");
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // Do nothing if there's no data to read
                }
                Err(e) => println!("encountered IO error: {e}"),
            }
    
            // Clear the buffer for the next iteration
            buf = [0; 100];
        }
    }).await;

    // tokio::join!(main)
    
}
*/

/*
###    Client sample console output    ###
Sent: Hi server, 0 to 127.0.0.1:8080
Received from server: 127.0.0.1:8080, message: hello client
Sent: Hi server, 1 to 127.0.0.1:8080
Received from server: 127.0.0.1:8080, message: server seperate broadcast
Sent: Hi server, 2 to 127.0.0.1:8080
Received from server: 127.0.0.1:8080, message: server seperate broadcast
Sent: Hi server, 3 to 127.0.0.1:8080
Received from server: 127.0.0.1:8080, message: server seperate broadcast
Sent: Hi server, 4 to 127.0.0.1:8080
Received from server: 127.0.0.1:8080, message: server seperate broadcast
Sent: Hi server, 5 to 127.0.0.1:8080
Received from server: 127.0.0.1:8080, message: server seperate broadcast
Sent: Hi server, 6 to 127.0.0.1:8080
Received from server: 127.0.0.1:8080, message: server seperate broadcast
Sent: Hi server, 7 to 127.0.0.1:8080
Received from server: 127.0.0.1:8080, message: server seperate broadcast
Sent: Hi server, 8 to 127.0.0.1:8080
Received from server: 127.0.0.1:8080, message: server seperate broadcast
Sent: Hi server, 9 to 127.0.0.1:8080
Received from server: 127.0.0.1:8080, message: server seperate broadcast
Sent: Hi server, 10 to 127.0.0.1:8080
Received from server: 127.0.0.1:8080, message: server seperate broadcast
Sent: Hi server, 11 to 127.0.0.1:8080
Received from server: 127.0.0.1:8080, message: server seperate broadcast
Sent: Hi server, 12 to 127.0.0.1:8080
Received from server: 127.0.0.1:8080, message: server seperate broadcast
Sent: Hi server, 13 to 127.0.0.1:8080
Received from server: 127.0.0.1:8080, message: server seperate broadcast
Sent: Hi server, 14 to 127.0.0.1:8080
Received from server: 127.0.0.1:8080, message: server seperate broadcast
Sent: Hi server, 15 to 127.0.0.1:8080
Received from server: 127.0.0.1:8080, message: server seperate broadcast
Sent: Hi server, 16 to 127.0.0.1:8080
Received from server: 127.0.0.1:8080, message: server seperate broadcast
Sent: Hi server, 17 to 127.0.0.1:8080
Received from server: 127.0.0.1:8080, message: server seperate broadcast
Sent: Hi server, 18 to 127.0.0.1:8080
Received from server: 127.0.0.1:8080, message: server seperate broadcast
Sent: Hi server, 19 to 127.0.0.1:8080
Received from server: 127.0.0.1:8080, message: server seperate broadcast
Sent: Hi server, 20 to 127.0.0.1:8080
Received from server: 127.0.0.1:8080, message: server seperate broadcast
Sent: Hi server, 21 to 127.0.0.1:8080
Received from server: 127.0.0.1:8080, message: server seperate broadcast
Sent: Hi server, 22 to 127.0.0.1:8080
Received from server: 127.0.0.1:8080, message: server seperate broadcast
Sent: Hi server, 23 to 127.0.0.1:8080
Received from server: 127.0.0.1:8080, message: server seperate broadcast
Sent: Hi server, 24 to 127.0.0.1:8080
Received from server: 127.0.0.1:8080, message: server seperate broadcast
Sent: Hi server, 25 to 127.0.0.1:8080
Received from server: 127.0.0.1:8080, message: server seperate broadcast


###    After removed the `_server_seperate_broadcast_channel` handle    ###
Received from server: 127.0.0.1:8080, message: hello client
Sent: Hi server, 6 to 127.0.0.1:8080
Received from server: 127.0.0.1:8080, message: hello client
Sent: Hi server, 7 to 127.0.0.1:8080
Received from server: 127.0.0.1:8080, message: hello client
Sent: Hi server, 8 to 127.0.0.1:8080
Received from server: 127.0.0.1:8080, message: hello client
Sent: Hi server, 9 to 127.0.0.1:8080
Received from server: 127.0.0.1:8080, message: hello client
Sent: Hi server, 10 to 127.0.0.1:8080
Received from server: 127.0.0.1:8080, message: hello client
Sent: Hi server, 11 to 127.0.0.1:8080
Received from server: 127.0.0.1:8080, message: hello client
Sent: Hi server, 12 to 127.0.0.1:8080
Received from server: 127.0.0.1:8080, message: hello client
Sent: Hi server, 13 to 127.0.0.1:8080
Received from server: 127.0.0.1:8080, message: hello client
Sent: Hi server, 14 to 127.0.0.1:8080
Received from server: 127.0.0.1:8080, message: hello client
Sent: Hi server, 15 to 127.0.0.1:8080
Received from server: 127.0.0.1:8080, message: hello client
Sent: Hi server, 16 to 127.0.0.1:8080
Received from server: 127.0.0.1:8080, message: hello client
Sent: Hi server, 17 to 127.0.0.1:8080
Received from server: 127.0.0.1:8080, message: hello client
Sent: Hi server, 18 to 127.0.0.1:8080
Received from server: 127.0.0.1:8080, message: hello client
Sent: Hi server, 19 to 127.0.0.1:8080
Received from server: 127.0.0.1:8080, message: hello client
Sent: Hi server, 20 to 127.0.0.1:8080
Received from server: 127.0.0.1:8080, message: hello client
Sent: Hi server, 21 to 127.0.0.1:8080
Received from server: 127.0.0.1:8080, message: hello client
Sent: Hi server, 22 to 127.0.0.1:8080
Received from server: 127.0.0.1:8080, message: hello client
Sent: Hi server, 23 to 127.0.0.1:8080
Received from server: 127.0.0.1:8080, message: hello client
Sent: Hi server, 24 to 127.0.0.1:8080
Received from server: 127.0.0.1:8080, message: hello client


###    Server output    ###
Received from 127.0.0.1:60255: Hi server, 0
Received from 127.0.0.1:60255: Hi server, 1
Received from 127.0.0.1:60255: Hi server, 2
Received from 127.0.0.1:60255: Hi server, 3
Received from 127.0.0.1:60255: Hi server, 4
Received from 127.0.0.1:60255: Hi server, 5
Received from 127.0.0.1:60255: Hi server, 6
Received from 127.0.0.1:60255: Hi server, 7
Received from 127.0.0.1:60255: Hi server, 8
Received from 127.0.0.1:60255: Hi server, 9
Received from 127.0.0.1:60255: Hi server, 10
Received from 127.0.0.1:60255: Hi server, 11
Received from 127.0.0.1:60255: Hi server, 12
Received from 127.0.0.1:60255: Hi server, 13
Received from 127.0.0.1:60255: Hi server, 14
Received from 127.0.0.1:60255: Hi server, 15
Received from 127.0.0.1:60255: Hi server, 16
Received from 127.0.0.1:60255: Hi server, 17
Received from 127.0.0.1:60255: Hi server, 18
Received from 127.0.0.1:60255: Hi server, 19
Received from 127.0.0.1:60255: Hi server, 20

*/





// try to spliting into 2 task with mpsc
// status: failed, no output in server
// the client single line output:   Sent: Hi server, 0 to 127.0.0.1:8080
/*
use std::{io, net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket}, sync::Arc, sync::atomic::{AtomicBool, Ordering}};
use tokio::sync::mpsc;
use tokio::task;

#[tokio::main]
async fn main() {
    let sock_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);

    let socket = UdpSocket::bind(sock_addr).expect("couldn't bind to address");
    socket.set_nonblocking(true).unwrap();

    // clonning the socket so that we have one for sending, one for receiving
    let socket_clone = socket.try_clone().expect("couldn't clone the socket");

    println!("UdpSocket opened on {}, listening for incoming messages", sock_addr);

    let (tx, mut rx) = mpsc::channel::<(Vec<u8>, SocketAddr)>(100);
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();

    let receive_task = tokio::spawn(async move {
        let mut buf = [0; 100];
        while running.load(Ordering::SeqCst) {
            match socket_clone.recv_from(&mut buf) {
                Ok((num_bytes_read, src_addr)) => {
                    let received_data = buf[..num_bytes_read].to_vec();
                    tx.send((received_data, src_addr)).await.expect("send error");
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // Do nothing if there's no data to read
                }
                Err(e) => println!("encountered IO error: {e}"),
            }
        }
    });

    let send_task = tokio::spawn(async move {
        while let Some((data, src_addr)) = rx.recv().await {
            // Process the received data and send response here
            let response_message = "hello client";
            socket.send_to(response_message.as_bytes(), src_addr).expect("send_to failed");
        }
    });

    tokio::join!(send_task, receive_task);

    // Wait for both tasks to finish
    // receive_task.await.expect("receive_task failed");
    // send_task.await.expect("send_task failed");

    println!("Server is shutting down...");
}
*/