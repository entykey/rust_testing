// https://www.youtube.com/watch?v=Iapc-qGTEBQ&list=PLNT92VogwJ9nlxnQBku7PUqQYkHxFoNXP&index=5&t=1814s
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};


#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    let (tx, rx) = broadcast::channel(10);

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0{
                            break;
                        }
                        tx.send((line.clone(), addr)).unwrap();
                        line.clear();
                    }
                    result = rx.recv() => {
                        let (msg, other_addr) = result.unwrap();

                        if addr != other_addr {
                            writer.write_all((msg.as_bytes())).await.unwrap();
                        }
                    }
                }
            }
        });

    }
}



/*
    match &cli.command {
        Commands::Connect { host, port } => {
            println!("connect to {}:{}", host, port);

            runtime.block_on(async {
                tokio::select! {
                    _ = stream::client() => {}
                    _ = tokio::signal::ctrl_c() => {}
                }
            });
        }
        Commands::Serve { bind_host, port } => {
            println!("bind to {}:{}", bind_host, port);

            runtime.block_on(async {
                tokio::select! {
                    _ = stream::server() => {}
                    _ = tokio::signal::ctrl_c() => {}
                }
            });
        }
    }

    // https://github.com/tokio-rs/tokio/issues/2318
    runtime.shutdown_timeout(Duration::from_secs(0));
    */