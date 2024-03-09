use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;

const NUM_WORKERS: usize = 4;
const NUM_TASKS: usize = 20;

fn main() {
    let (task_tx, task_rx) = mpsc::channel();
    let task_rx = Arc::new(Mutex::new(task_rx));

    let (result_tx, result_rx) = mpsc::channel();

    // Create worker threads
    for id in 0..NUM_WORKERS {
        let task_rx = task_rx.clone();
        let result_tx = result_tx.clone();

        thread::spawn(move || {
            println!("Worker {} is waiting for tasks.", id);

            loop {
                // Get the next task from the task channel
                let task = match task_rx.lock().unwrap().recv() {
                    Ok(task) => task,
                    Err(_) => {
                        // The channel is closed, so the worker should exit
                        println!("Worker {} is exiting.", id);
                        break;
                    }
                };

                // Process the task
                println!("Worker {} is processing task: {}", id, task);

                // Simulate some work
                thread::sleep(std::time::Duration::from_secs(1));

                // Send the result back to the result channel
                result_tx.send(format!("Result of task {}: done", task)).unwrap();
            }
        });
    }

    // Submit tasks to the worker pool
    for task_id in 0..NUM_TASKS {
        task_tx.send(task_id).unwrap();
    }

    // Close the task channel to signal the workers to exit
    drop(task_tx);

    // Collect results from the workers
    for _ in 0..NUM_TASKS {
        let result = result_rx.recv().unwrap();
        println!("Received result: {}", result);
    }
}
