/*
// this code deliberately produces potential data race for those who don't know what data race is
use std::thread;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};


#[allow(dead_code)]
fn main() {

    let main_time: std::time::Instant = std::time::Instant::now();

    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // UNSAFE: Data race occurs due to lack of proper synchronization
            let value = counter_clone.load(Ordering::Relaxed);
            counter_clone.store(value + 1, Ordering::Relaxed);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // After all threads have finished, we read the final value
    // of the counter.
    let final_value = counter.load(Ordering::Relaxed);
    println!("Final value of the counter: {}", final_value);

    // => Sometimes returns 9, somtimes returns 10 (data race)
    
    // end of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\nExecution time: {:?} ({:?} ms)", duration, elapsed_ms);

}
*/


// To fix the data race, you need to use proper synchronization mechanisms, 
// such as Ordering::SeqCst, which provides sequential consistency, ensuring that 
// the operations are properly ordered and synchronized across all threads:

use std::thread;
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {
    let main_time: std::time::Instant = std::time::Instant::now();

    // for AtomicUsize (lock-free)
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for _ in 0..10 {    // 0..=9
        //println!("thread num: {}", i); // printing in loop cause considerable performance decrement !! (0.5ms to 1ms)
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // Proper synchronization with Ordering::SeqCst
            // SeqCst: guarantee that all threads see all sequentially consistent operations in the same order.
            let value = counter_clone.load(Ordering::SeqCst);
            counter_clone.store(value + 1, Ordering::SeqCst);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // After all threads have finished, we read the final value
    // of the counter.
    let final_value = counter.load(Ordering::SeqCst);
    println!("Final value of the counter: {}", final_value);



    // now for shared vector (lock-free, avoid data race)
    // `Mutex`: a mutual exclusion lock that allows only one thread to access the shared data at a time. 
    // `RwLock`: a read-write lock that allows multiple threads to read the shared data simultaneously, but only one thread can have exclusive write access to modify the data.
    let vec_data: Arc<RwLock<Vec<i32>>> = Arc::new(RwLock::new(Vec::new()));
    let mut handles: Vec<thread::JoinHandle<()>> = vec![];

    for i in 0..10 {
        let vec_data_clone: Arc<RwLock<Vec<i32>>> = Arc::clone(&vec_data);
        let handle = thread::spawn(move || {
            // Simulate multiple threads concurrently accessing and modifying the vector
            let mut data: std::sync::RwLockWriteGuard<'_, Vec<i32>> = vec_data_clone.write().unwrap();
            data.push(i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // After all threads have finished, read the final vector data
    let data: std::sync::RwLockReadGuard<'_, Vec<i32>> = vec_data.read().unwrap();
    println!("Final vector: {:?}", *data);


    // end of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\n⌛️ Execution time: {:?} ({:?} ms)", duration, elapsed_ms);
}
