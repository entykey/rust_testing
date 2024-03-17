//! async_std print file with dirs crate
use std::env::args;

use async_std::fs::File;
use async_std::io;
use async_std::prelude::*;
use async_std::task;
use dirs;

const LEN: usize = 16 * 1024; // 16 Kb

fn main() -> io::Result<()> {
    
    // retrieve "Users/user/Documents" directory on macos
    let path = dirs::document_dir().unwrap().join("example.txt"); // Replace with the name you want to save the file as
    // let path = args().nth(1).expect("missing path argument");

    task::block_on(async {
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
    })
}