use rand::Rng;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use tokio::task::spawn;

async fn producer(tx: mpsc::Sender<i32>) {
    let mut rng = rand::thread_rng();
    for i in 0..10 {
        let delay = rng.gen_range(100..=1000);
        tokio::task::block_in_place(|| {
            // Simulate some work being done by sleeping for the random duration.
            thread::sleep(Duration::from_millis(delay));
        });
        tx.send(i).unwrap();
    }
}

async fn consumer(rx: mpsc::Receiver<i32>) {
    while let Ok(received_value) = rx.recv() {
        println!("Received value: {}", received_value);
    }
}

#[tokio::main]
async fn main() {
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    let consumer_task1 = tokio::spawn(consumer(rx1));
    let consumer_task2 = tokio::spawn(consumer(rx2));

    tokio::spawn(producer(tx1));
    tokio::spawn(producer(tx2));

    // Use tokio::select! to wait for both consumer tasks to finish.
    tokio::select! {
        _ = consumer_task1 => {},
        _ = consumer_task2 => {},
    }
}
