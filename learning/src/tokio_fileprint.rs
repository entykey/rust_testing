/*
// attempt 1
// STATUS: The breakline happen before the stdout task.
// tokio print file with dirs crate
use std::env::args;

use dirs;

use tokio::{fs::File, io::{self, AsyncReadExt, AsyncWriteExt}};

const LEN: usize = 16 * 1024; // 16 Kb

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    
    // retrieve "Users/user/Documents" directory on macos
    let path = dirs::document_dir().unwrap().join("example.txt"); // Replace with the name you want to save the file as
    // let path = args().nth(1).expect("missing path argument");

    tokio::spawn(async move {
        let mut file = File::open(&path).await?;
        let mut stdout = io::stdout();
        let mut buf = vec![0u8; LEN];

        loop {
            // Read a buffer from the file.
            let n = file.read(&mut buf).await?;

            // If this is the end of file, clean up and return.
            if n == 0 {
                stdout.flush().await?;
                return Ok(());
            }

            // Write the buffer into stdout.
            stdout.write_all(&buf[..n]).await?;
            println!("\n"); // breakline in terminal
        }
    }).await?
}
*/




/*
// attempt 2
// STATUS: program wont stop after finish
// tokio print file with dirs crate
use std::env::args;

use dirs;

use tokio::{fs::File, io::{self, AsyncReadExt, AsyncWriteExt}};

const LEN: usize = 16 * 1024; // 16 Kb

#[tokio::main]
async fn main() {
    
    // retrieve "Users/user/Documents" directory on macOS
    let path = dirs::document_dir().unwrap().join("example.txt"); // Replace with the name you want to save the file as
    // let path = args().nth(1).expect("missing path argument");

    // Spawn a Tokio task to read the file asynchronously.
    let reading_task = tokio::spawn(async move {
        let mut file = File::open(&path).await.unwrap();
        let mut stdout = io::stdout();
        let mut buf = vec![0u8; LEN];

        loop {
            // Read a buffer from the file.
            let n = file.read(&mut buf).await.unwrap();

            // If this is the end of file, clean up and return.
            if n == 0 {
                stdout.flush().await.unwrap();
                // return Ok(());
            }

            // Write the buffer into stdout.
            stdout.write_all(&buf[..n]).await.unwrap();
            // println!("\n"); // breakline in terminal
        }
    });

    // Await the completion of the spawned Tokio task.
    reading_task.await.unwrap();

    // Ok(())
}
*/





/*
// attempt 3
// STATUS: worked, but the the breakline is being executed before the stdout is fully flushed.
// explanatory: In an asynchronous Rust program using Tokio, we typically want the program to terminate when all tasks are completed. To achieve this, we can use a synchronization mechanism like channels to signal when the file reading task is finished. Here's how we can modify our code to use channels
use tokio::{fs::File, io::{self, AsyncReadExt, AsyncWriteExt}, sync::mpsc};

const LEN: usize = 16 * 1024; // 16 Kb

#[tokio::main]
async fn main() {
    // retrieve "Users/user/Documents" directory on macOS
    let path = dirs::document_dir().unwrap().join("example.txt");

    // Create a channel to signal when the file reading task is finished.
    let (tx, mut rx) = mpsc::channel::<()>(1);

    // Spawn a Tokio task to read the file asynchronously.
    let reading_task = tokio::spawn(async move {
        let mut file = File::open(&path).await.unwrap();
        let mut stdout = io::stdout();
        let mut buf = vec![0u8; LEN];

        loop {
            // Read a buffer from the file.
            let n = file.read(&mut buf).await.unwrap();

            // If this is the end of file, clean up and return.
            if n == 0 {
                stdout.flush().await.unwrap();
                break;
            }

            // Write the buffer into stdout.
            stdout.write_all(&buf[..n]).await.unwrap();
            // Add a breakline after printing the content.
            println!();
        }

        // Send a signal through the channel to indicate that the task is finished.
        let _ = tx.send(()).await;
    });

    // Wait for the file reading task to finish.
    reading_task.await.unwrap();

    // Wait for the signal from the channel indicating that the task is finished.
    rx.recv().await;

    // The program will automatically terminate here.
}
*/





// STATUS: solved
// to ensure that the breakline is printed after all the content has been flushed to stdout. We can achieve this by waiting for the file reading task to finish before printing the breakline.
use tokio::{fs::File, io::{self, AsyncReadExt, AsyncWriteExt}, sync::mpsc};

const LEN: usize = 16 * 1024; // 16 Kb

#[tokio::main]
async fn main() {
    // retrieve "Users/user/Documents" directory on macOS
    let path = dirs::document_dir().unwrap().join("example.txt");

    // Create a channel to signal when the file reading task is finished.
    let (tx, mut rx) = mpsc::channel::<()>(1);

    // Spawn a Tokio task to read the file asynchronously.
    let reading_task = tokio::spawn(async move {
        let mut file = File::open(&path).await.unwrap();
        let mut stdout = io::stdout();
        let mut buf = vec![0u8; LEN];

        loop {
            // Read a buffer from the file.
            let n = file.read(&mut buf).await.unwrap();

            // If this is the end of file, clean up and return.
            if n == 0 {
                stdout.flush().await.unwrap();
                break;
            }

            // Write the buffer into stdout.
            stdout.write_all(&buf[..n]).await.unwrap();
        }

        // Send a signal through the channel to indicate that the task is finished.
        let _ = tx.send(()).await;
    });

    // Wait for the file reading task to finish.
    reading_task.await.unwrap();

    // Wait for the signal from the channel indicating that the task is finished.
    rx.recv().await;

    // Add a breakline after the task is finished.
    println!();
}
