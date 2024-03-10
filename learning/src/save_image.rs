extern crate dirs;
extern crate reqwest;

// use async_std::fs::File;
// use async_std::prelude::*;
// use async_std::task;
use std::io;
use tokio::{fs::File, io::AsyncWriteExt};
// $ cargo add reqwest
// $ cargo add dirs

async fn fetch_image_data(url: &str) -> io::Result<Vec<u8>> {
    let response = reqwest::get(url).await.unwrap();
    let bytes = response.bytes().await.unwrap();
    Ok(bytes.to_vec())
}

async fn save_image_data_to_downloads(image_data: Vec<u8>) -> io::Result<()> {

    // Save to Mac Downloads directory "/Users/user/Downloads"
    let downloads_dir = dirs::download_dir().ok_or(io::Error::new(
        io::ErrorKind::NotFound,
        "Download directory not found",
    ))?;
    let file_path = downloads_dir.join("dog_image.jpg");
    let mut file = File::create(file_path).await?;
    file.write_all(&image_data).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let url = "https://images.dog.ceo/breeds/mountain-swiss/n02107574_1387.jpg";
    match fetch_image_data(url).await {
        Ok(image_data) => {
            if let Err(err) = save_image_data_to_downloads(image_data).await {
                eprintln!("Error saving image: {}", err);
            } else {
                println!("Image saved to Downloads directory");
            }
        }
        Err(err) => {
            eprintln!("Error fetching image data: {}", err);
        }
    }
}
/*
// async_std main()
fn main() {
    task::block_on(async {
        let url = "https://images.dog.ceo/breeds/mountain-swiss/n02107574_1387.jpg";
        match fetch_image_data(url).await {
            Ok(image_data) => {
                if let Err(err) = save_image_data_to_downloads(image_data).await {
                    eprintln!("Error saving image: {}", err);
                } else {
                    println!("Image saved to Downloads directory");
                }
            }
            Err(err) => {
                eprintln!("Error fetching image data: {}", err);
            }
        }
    });
}
*/



// new code, with indicatif progress:
//*
// Explanatory:  This happens because response.bytes() returns a Future that yields a single Result with the bytes of the response body.
// To fix this error, you should use a different approach to read the response body. We can use response.bytes() to fetch the entire response body as bytes and then process it accordingly.
// NOTE: I always got the error from chatgpt when use "?" instead of .unwrap() : the trait `From<reqwest::Error>` is not implemented for `std::io::Error`
use reqwest;
use std::fs::File;
use std::io::{self, prelude::*};
use std::path::PathBuf;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};

async fn async_read_with_progress(url: &str, file_path: PathBuf) -> io::Result<()> {
    println!("Connecting to host...");
    let response = reqwest::get(url).await.unwrap();
    if response.status().is_success() {
        println!("Successfully connected to host.");
    } else {
        eprintln!("Failed to connect to host: {}", response.status());
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to connect to host",
        ));
    }

    let total_size = response.content_length().unwrap_or(0);
    println!("file size: {}", total_size);
    let mut downloaded = 0;

    let pb = ProgressBar::new(total_size);
    // pb.set_style(
    //     ProgressStyle::default_bar()
    //         .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
    //         .progress_chars("#>-"),
    // );
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        // .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));

    let mut writer = File::create(&file_path)?;

    let bytes = response.bytes().await.unwrap();
    writer.write_all(&bytes)?;

    pb.finish_with_message("Downloaded");
    Ok(())
}

#[tokio::main]
async fn main() {
    let url = "https://images.dog.ceo/breeds/mountain-swiss/n02107574_1387.jpg"; // Replace with the actual URL of the file
    let file_path = dirs::download_dir().unwrap().join("dog_image.jpg"); // Replace with the name you want to save the file as
    match async_read_with_progress(url, file_path).await {
        Ok(_) => println!("File downloaded successfully."),
        Err(err) => eprintln!("Error downloading file: {}", err),
    }
}
// */v