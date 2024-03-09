//use tokio::task;
use std::time::Duration;

async fn compute_square_async(n: i32) -> i32 {
    // Simulate a time-consuming computation
    tokio::time::sleep(Duration::from_millis(200)).await;
    n * n
}

#[tokio::main]
async fn main() {
    let nums = vec![1, 2, 3, 4, 5];

    let tasks: Vec<_> = nums.into_iter().map(compute_square_async).collect();

    for (i, task) in tasks.into_iter().enumerate() {
        let result = task.await;
        println!("Computation for {} yielded result: {}", i + 1, result);
    }
}



/*
use std::thread;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;

fn compute_square(n: i32) -> i32 {
    n * n
}

fn main() {
    let nums = Arc::new(Mutex::new(vec![1, 2, 3, 4, 5]));
    let (sender, receiver) = mpsc::channel();

    // temporary value and its usage within the thread. To resolve this, 
    // we need to ensure that the value remains valid throughout the entire thread's execution.
    let thread_handles: Vec<_> = Arc::clone(&nums)
        .lock()
        .unwrap()
        .iter()
        .map(|num| {
            let sender_clone = sender.clone();
            let num = *num; // Make a copy of the value to use in the thread
            thread::spawn(move || {
                let result = compute_square(num);
                // Send the result back to the main thread
                sender_clone.send(result).unwrap();
            })
        })
        .collect();

    // Collect the results from the threads
    let results: Vec<i32> = receiver.iter().take(nums.lock().unwrap().len()).collect();

    // Wait for all threads to complete
    for handle in thread_handles {
        handle.join().unwrap();
    }

    println!("Results: {:?}", results);
}
*/

/*
use std::thread;
use std::time::Duration;

fn compute_square(n: i32) -> i32 {
    n * n
}

fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    let mut handles = vec![];

    for num in nums {
        let handle = thread::spawn(move || {
            // Perform some computation in the background
            compute_square(num)
        });
        handles.push(handle);
    }

    // Wait for all threads to complete and collect the results
    let results: Vec<i32> = handles.into_iter().map(|handle| handle.join().unwrap()).collect();

    println!("Results: {:?}", results);
}

*/