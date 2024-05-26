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
// mpsc_worker
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;

const NUM_WORKERS: u8 = 4;
const NUM_TASKS: u8 = 20;

fn main() {
    let (task_tx, task_rx) = mpsc::channel();
    let task_rx: Arc<Mutex<mpsc::Receiver<usize>>> = Arc::new(Mutex::new(task_rx));

    let (result_tx, result_rx) = mpsc::channel();

    // Create worker threads
    for id in 0..NUM_WORKERS {
        let task_rx: Arc<Mutex<mpsc::Receiver<usize>>> = task_rx.clone();
        let result_tx: mpsc::Sender<String> = result_tx.clone();

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
                thread::sleep(std::time::Duration::from_millis(200));

                // Send the result back to the result channel
                result_tx.send(format!("Result of task {}: done", task)).unwrap();
            }
        });
    }

    // Submit tasks to the worker pool
    for task_id in 0..NUM_TASKS {
        task_tx.send(task_id.into()).unwrap();
    }

    // Close the task channel to signal the workers to exit
    drop(task_tx);

    // Collect results from the workers
    for _ in 0..NUM_TASKS {
        let result: String = result_rx.recv().unwrap();
        println!("Received result: {}", result);
    }
}
*/



/*
use std::sync::{Mutex, Arc};
use std::thread;

fn safe_increment(g_i: &Arc<Mutex<i32>>, iterations: i32) {

    // Safe part: Lock the mutex before modifying the shared variable
    let mut g_i = g_i.lock().unwrap(); // Lock the mutex
    for _ in 0..iterations {
        *g_i += 1; // Modify the shared variable
    }

    // Print the value of the shared variable
    println!("thread {:?}, g_i: {}", thread::current().id(), *g_i);
    // Mutex is automatically released when 'g_i' goes out of scope
}

fn unsafe_increment(g_i: &Arc<Mutex<i32>>, iterations: i32) {

    // Unsafe part: Modifying the shared variable without locking the mutex
    for _ in 0..iterations {
        // Lock the mutex before modifying the shared variable (unsafe)
        let mut g_i = g_i.lock().unwrap();
        *g_i += 1; // Modify the shared variable
    }

    // Print the value of the shared variable
    println!("thread {:?}, g_i: {}", thread::current().id(), g_i.lock().unwrap());
    // Mutex is automatically released when 'g_i' goes out of scope
}

fn main() {

    // Create a shared variable 'g_i' protected by a mutex
    let g_i = Arc::new(Mutex::new(0));

    // Safe increment operation
    println!("safe_increment:");
    {
        let g_i_clone = g_i.clone(); // Clone 'g_i' for each thread

        // Spawn two threads to execute the 'safe_increment' function concurrently
        let t1 = thread::spawn(move || safe_increment(&g_i_clone, 1_000_000));
        let g_i_clone = g_i.clone(); // Clone 'g_i' for each thread
        let t2 = thread::spawn(move || safe_increment(&g_i_clone, 1_000_000));

        // Wait for both threads to finish
        t1.join().unwrap();
        t2.join().unwrap();
    }

    // Print the final value of the shared variable
    println!("after, g_i: {}\n", *g_i.lock().unwrap());

    // Unsafe increment operation
    println!("unsafe_increment:");
    {
        let g_i_clone = g_i.clone(); // Clone 'g_i' for each thread

        // Spawn two threads to execute the 'unsafe_increment' function concurrently
        let t1 = thread::spawn(move || unsafe_increment(&g_i_clone, 1_000_000));
        let g_i_clone = g_i.clone(); // Clone 'g_i' for each thread
        let t2 = thread::spawn(move || unsafe_increment(&g_i_clone, 1_000_000));

        // Wait for both threads to finish
        t1.join().unwrap();
        t2.join().unwrap();
    }

    // Print the final value of the shared variable
    println!("after, g_i: {}\n", *g_i.lock().unwrap());
}
*/
/*
Output:
safe_increment:
thread ThreadId(2), g_i: 1000000
thread ThreadId(3), g_i: 2000000
after, g_i: 2000000

unsafe_increment:
thread ThreadId(5), g_i: 3915662
thread ThreadId(4), g_i: 4000000
after, g_i: 4000000
*/



/*
// does not cause deadlock yet
use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    // Create a mutex-protected variable
    let counter = Arc::new(Mutex::new(0));

    // Clone the Arc for each thread
    let counter1 = Arc::clone(&counter);
    let counter2 = Arc::clone(&counter);

    // Spawn two threads that try to acquire the mutex lock simultaneously
    let t1 = thread::spawn(move || {
        let mut data = counter1.lock().unwrap();
        println!("Thread 1 acquired lock, waiting...");
        thread::sleep(std::time::Duration::from_secs(1));
        *data += 1;
        println!("Thread 1 incremented counter to {}", *data);
    });

    let t2 = thread::spawn(move || {
        let mut data = counter2.lock().unwrap();
        println!("Thread 2 acquired lock, waiting...");
        thread::sleep(std::time::Duration::from_secs(1));
        *data += 1;
        println!("Thread 2 incremented counter to {}", *data);
    });

    // Wait for both threads to finish
    t1.join().unwrap();
    t2.join().unwrap();
}
*/



/*
// deadlock example in Rust
// In this example, Thread 1 acquires the lock and holds it indefinitely using a loop. Meanwhile, Thread 2 attempts to acquire the lock after a short delay. Since Thread 1 never releases the lock, Thread 2 will block indefinitely, leading to a deadlock scenario.
use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    // Create a mutex-protected variable
    let counter = Arc::new(Mutex::new(0));

    // Clone the Arc for each thread
    let counter1 = Arc::clone(&counter);
    let counter2 = Arc::clone(&counter);

    // Spawn two threads
    let t1 = thread::spawn(move || {
        let mut data = counter1.lock().unwrap();
        println!("Thread 1 acquired lock");
        // Simulate holding the lock indefinitely
        loop {}
    });

    let t2 = thread::spawn(move || {
        // Wait for a short period to ensure thread 1 acquires the lock first
        thread::sleep(std::time::Duration::from_secs(1));
        println!("Thread 2 attempting to acquire lock");
        // Attempt to acquire the lock, which will block indefinitely due to deadlock
        let _data = counter2.lock().unwrap();
        println!("Thread 2 acquired lock");
    });

    // Wait for both threads to finish
    t1.join().unwrap();
    t2.join().unwrap();
}
*/
/*
Output (deadlock):
Thread 1 acquired lock
Thread 2 attempting to acquire lock
*/



/*
use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    // Create two mutex-protected variables
    let counter1 = Arc::new(Mutex::new(0));
    let counter2 = Arc::new(Mutex::new(0));

    // Clone the Arc for each thread
    let counter1_clone = Arc::clone(&counter1);
    let counter2_clone = Arc::clone(&counter2);

    // Spawn two threads
    let t1 = thread::spawn(move || {
        // Acquire lock on counter1 first
        let _lock1 = counter1_clone.lock().unwrap();
        println!("Thread 1 acquired lock on counter1");

        // Simulate some workc
        thread::sleep(std::time::Duration::from_secs(1));

        // Acquire lock on counter2 next
        let _lock2 = counter2_clone.lock().unwrap();
        println!("Thread 1 acquired lock on counter2");
    });

    let t2 = thread::spawn(move || {
        // Acquire lock on counter1 first (same order as thread 1)
        let _lock1 = counter1.lock().unwrap();
        println!("Thread 2 acquired lock on counter1");

        // Simulate some work
        thread::sleep(std::time::Duration::from_secs(1));

        // Acquire lock on counter2 next
        let _lock2 = counter2.lock().unwrap();
        println!("Thread 2 acquired lock on counter2");
    });

    // Wait for both threads to finish
    t1.join().unwrap();
    t2.join().unwrap();
}
*/
/*
Output:
Thread 1 acquired lock on counter1
Thread 1 acquired lock on counter2
Thread 2 acquired lock on counter1
Thread 2 acquired lock on counter2
*/



// Here's the modified code with additional messages indicating when a thread is waiting to acquire a lock
/*
use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    // Create two mutex-protected variables
    let counter1 = Arc::new(Mutex::new(0));
    let counter2 = Arc::new(Mutex::new(0));

    // Clone the Arc for each thread
    let counter1_clone = Arc::clone(&counter1);
    let counter2_clone = Arc::clone(&counter2);

    // Spawn two threads
    let t1 = thread::spawn(move || {
        // Acquire lock on counter1 first
        let _lock1 = counter1_clone.lock().unwrap();
        println!("Thread 1 acquired lock on counter1");

        // Simulate some work
        thread::sleep(std::time::Duration::from_secs(1));

        // Acquire lock on counter2 next
        println!("Thread 1 waiting to acquire lock on counter2");
        let _lock2 = counter2_clone.lock().unwrap();
        println!("Thread 1 acquired lock on counter2");
    });

    let t2 = thread::spawn(move || {
        // Acquire lock on counter1 first (same order as thread 1)
        let _lock1 = counter1.lock().unwrap();
        println!("Thread 2 acquired lock on counter1");

        // Simulate some work
        thread::sleep(std::time::Duration::from_secs(1));

        // Acquire lock on counter2 next
        println!("Thread 2 waiting to acquire lock on counter2");
        let _lock2 = counter2.lock().unwrap();
        println!("Thread 2 acquired lock on counter2");
    });

    // Wait for both threads to finish
    t1.join().unwrap();
    t2.join().unwrap();
}
*/
/*
Output:
Thread 1 acquired lock on counter1
Thread 1 waiting to acquire lock on counter2
Thread 1 acquired lock on counter2
Thread 2 acquired lock on counter1
Thread 2 waiting to acquire lock on counter2
Thread 2 acquired lock on counter2
*/



/*
use std::sync::{Arc, RwLock};
use std::thread;

#[tokio::main]
async fn main() {
    let main_time: tokio::time::Instant = tokio::time::Instant::now();
    let count = Arc::new(RwLock::new(0));
    let num = Arc::new(RwLock::new(1));
    let odds: Arc<RwLock<Vec<u64>>> = Arc::new(RwLock::new(Vec::new()));

    let mut threads = vec![];

    for _ in 0..10 {
        let count = Arc::clone(&count);
        let num = Arc::clone(&num);
        let odds = Arc::clone(&odds);

        let handle = thread::spawn(move || {
            loop {
                let mut count = count.write().unwrap();
                let mut num = num.write().unwrap();
                let mut odds = odds.write().unwrap();

                if *count == 100 {
                    break;
                }

                *count += 1;
                odds.push(*num);

                *num += 2; // Increment by 2 to get next odd number
            }
        });

        threads.push(handle);
    }

    for handle in threads {
        handle.join().unwrap();
    }

    println!("All threads finished!");

    // End of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\n⌛️ Execution time: {:?} ({:?} ms)", duration, elapsed_ms);

    let odds = odds.read().unwrap();
    println!("First 100 odd numbers: {:?}", odds);
}
// 1.33ms, 714.448µs (0.714448 ms), 898.848µs (0.8988480000000001 ms), 697.308µs (0.697308 ms)
*/



/*
// STATUS: this code cause a data race due to lacking of proper synchronization
// due to problem of a low-knowledge person: https://stackoverflow.com/questions/28800121/what-do-i-have-to-do-to-solve-a-use-of-moved-value-error
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {

    let mut count = 0;
    let mut num = 1;
    // let mut odds = Vec::new();
    let odds = Arc::new(Mutex::new(Vec::new()));
    let odds_clone = Arc::clone(&odds);

    let mut tasks = vec![];

    for _ in 0..10 {

        let odds = Arc::clone(&odds_clone);

        let task= thread::spawn(move || {

            let odds_clone = Arc::clone(&odds);

            loop {

                let mut odds_guard = odds_clone.lock().unwrap();

                if count == 10 {
                    break;
                }

                count += 1;
                odds_guard.push(num);

                num += 2; // Increment by 2 to get next odd number
            }
        });

        tasks.push(task);
    }

    for task in tasks {  // task: JoinHandle<()>
        task.join().unwrap();
    }

    println!("All tasks finished!");

    let odds = odds.lock().unwrap();
    println!("First 10 odd numbers: {:?}", odds);
}
*/


/*
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let main_time: tokio::time::Instant = tokio::time::Instant::now();

    let count = Arc::new(Mutex::new(0));    // Arc<Mutex<i32>>
    let num = Arc::new(Mutex::new(1));
    let odds = Arc::new(Mutex::new(Vec::new()));    // Arc<Mutex<Vec<i32>>>

    let mut tasks = vec![];

    for _ in 0..10 {
        let count = Arc::clone(&count);
        let num = Arc::clone(&num);
        let odds = Arc::clone(&odds);

        let task = thread::spawn(move || {
            let mut count = count.lock().unwrap();
            let mut num = num.lock().unwrap();
            let mut odds = odds.lock().unwrap();

            loop {
                if *count == 40 {
                    break;
                }

                *count += 1;
                odds.push(*num);

                *num += 2; // Increment by 2 to get next odd number
            }
        });

        tasks.push(task);
    }

    for task in tasks {
        task.join().unwrap();
    }

    println!("All tasks finished!");

    // End of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\n⌛️ Execution time: {:?} ({:?} ms)", duration, elapsed_ms);

    let odds = odds.lock().unwrap();
    println!("First 40 odd numbers: {:?}", *odds);
}
*/




/*
// working now
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};   // <-- changed from std::sync::RwLock to tokio::sync::RwLock

#[tokio::main]
async fn main() {
    let main_time: tokio::time::Instant = tokio::time::Instant::now();


    let count: Arc<RwLock<i32>> = Arc::new(RwLock::new(0));
    let num: Arc<RwLock<u64>> = Arc::new(RwLock::new(1));
    let odds: Arc<Mutex<Vec<u64>>> = Arc::new(Mutex::new(Vec::new()));

    let mut tasks: Vec<tokio::task::JoinHandle<()>> = vec![];

    for _ in 0..10 {
        let count: Arc<RwLock<i32>> = Arc::clone(&count);
        let num: Arc<RwLock<u64>> = Arc::clone(&num);
        let odds: Arc<Mutex<Vec<u64>>> = Arc::clone(&odds);

        let task: tokio::task::JoinHandle<()> = tokio::spawn(async move {

            let mut odds = odds.lock().await;
            let mut count: tokio::sync::RwLockWriteGuard<'_, i32> = count.write().await;
            let mut num: tokio::sync::RwLockWriteGuard<'_, u64> = num.write().await;

            loop {
                
                if *count == 40 {
                    break;
                }

                *count += 1;
                odds.push(*num);

                *num += 2; // Increment by 2 to get next odd number
            }
        });

        tasks.push(task);
    }

    for task in tasks {  // task: JoinHandle<()>
        task.await.unwrap();
    }

    println!("All tasks finished!");

    // End of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\n⌛️ Execution time: {:?} ({:?} ms)", duration, elapsed_ms);

    let odds: tokio::sync::MutexGuard<'_, Vec<u64>> = odds.lock().await;
    println!("First 40 odd numbers: {:?}", *odds);
}
*/
/*
Output:
All tasks finished!

⌛️ Execution time: 676.69µs (0.67669 ms)
First 100 odd numbers: [1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39, 41, 43, 45, 47, 49, 51, 53, 55, 57, 59, 61, 63, 65, 67, 69, 71, 73, 75, 77, 79, 81, 83, 85, 87, 89, 91, 93, 95, 97, 99, 101, 103, 105, 107, 109, 111, 113, 115, 117, 119, 121, 123, 125, 127, 129, 131, 133, 135, 137, 139, 141, 143, 145, 147, 149, 151, 153, 155, 157, 159, 161, 163, 165, 167, 169, 171, 173, 175, 177, 179, 181, 183, 185, 187, 189, 191, 193, 195, 197, 199]
*/



/*
// problem: https://stackoverflow.com/questions/73368724/the-trait-stdmarkersync-is-not-implemented-for-stdsyncmpscsenderi3?rq=3
use futures::stream::{self, StreamExt};
use std::sync::mpsc::{channel, Receiver, Sender};

#[tokio::main]
async fn main() {
    // let (tx, rx): (Sender<i32>, Receiver<i32>) = channel();

    // tokio::spawn(async move {
    //     let a = [1, 2, 3];
    //     let mut s = stream::iter(a.iter())
    //         .cycle()
    //         .for_each(move |int| async {
    //             tx.send(*int);
    //         })
    //         .await;
    // });
    
    
    // solution:
    let (tx, _rx): (Sender<i32>, Receiver<i32>) = channel();

    tokio::spawn(async move {
        let a = [1, 2, 3];
        let _s = stream::iter(a.iter())
            .cycle()
            .for_each(move |int| {
                let tx = tx.clone();
                async move {
                    tx.send(*int).unwrap();
                }
            })
            .await;
    });
}
*/


// use std::future::IntoFuture;

// fn print_type_name<'a, T>(input: &T) {
//     println!("Type of given variable is : {}",  std::any::type_name::<T>());
// }
// #[tokio::main]
// async fn main() {
//     let v = async { "meow" };
//     let mut fut = v.into_future();	 
//     // assert_eq!("meow", fut.await);
//     print_type_name(&fut);
//     println!("{}", fut.await);
// }


/*
// https://users.rust-lang.org/t/the-idiomatic-way-to-implement-a-future/67856
use futures::Future;
use std::{
    pin::Pin,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    task::{Context, Poll},
    thread,
};

struct MyFuture {
    started: bool,
    done: Arc<AtomicBool>,
    id: u8,
}

impl Future for MyFuture {

    type Output = bool;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("poll {}", self.id);
        if self.done.load(Ordering::Relaxed) {
            Poll::Ready(true)
        } else {
            if !self.started {
                let done = self.done.clone();
                let id = self.id;

                thread::spawn(move || {
                    thread::sleep(std::time::Duration::from_millis(5));
                    println!("work {:02} done", id);
                    done.store(true, Ordering::Relaxed);
                    // cx.waker().wake_by_ref();   // B
                });
                self.started = true;
            }
            cx.waker().wake_by_ref(); // A
            Poll::Pending
        }
    }
}

fn simulate(id: u8) -> MyFuture {
    let done = Arc::new(AtomicBool::new(false));
    MyFuture {
        started: false,
        done,
        id,
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let f = (0..2).map(|x| tokio::spawn(simulate(x)));
    for f in f {
        f.await;
    }
}
*/



/*
// https://www.codecentric.de/wissens-hub/blog/rust-einstieg
// Rust supports polymorphism – even without classes
// The following code example shows how to implement the trait Summaryfor the three types Point, Vec, and . LinkedListThe type of the elements of Vecand LinkedListis a generic parameter that ToStringmust implement the trait from the standard library. The method defined in this trait to_stringis summaryused to convert the elements of the vector or list into a string.
use std::collections::LinkedList;
use std::iter::FromIterator;

trait Summary {
    fn summary(&self) -> String;
}

struct Point {
    x: i32,
    y: i32,
}

impl Summary for Point {
    fn summary(&self) -> String {
        format!("Point({}, {})", self.x, self.y)
    }
}
impl<T: ToString> Summary for Vec<T> {
    fn summary(&self) -> String {
        format!("Vec({}): {}", self.len(), self.iter().map(|e| e.to_string()).collect::<String>())
    }
}
impl<T: ToString> Summary for LinkedList<T> {
    fn summary(&self) -> String {
        format!("LinkedList({}): {}", self.len(), self.iter().map(|e| e.to_string()).collect::<String>())
    }
}

fn main() {
    let point = Point { x: 5, y: -7 };
    let vec = vec![1, 2, 3, 4, 5];
    let list = LinkedList::from_iter(["a","b","c"].iter());

    let summaries: Vec<&dyn Summary> = vec![&point, &vec, &list];
    summaries.iter().for_each(|e| println!("{:?}", e.summary()));
}
*/




/*
// https://zenn.dev/pyteyon/scraps/ef87d60c1acb76
// https://github.com/cfsamson/examples-futures/blob/master/src/main.rs
fn main() {
    let start = Instant::now();
    let reactor = Reactor::new();

    let fut1 = async {
        let val = Task::new(reactor.clone(), 1, 1).await;
        println!("Got {} at time: {:.2}.", val, start.elapsed().as_secs_f32());
    };

    let fut2 = async {
        let val = Task::new(reactor.clone(), 2, 2).await;
        println!("Got {} at time: {:.2}.", val, start.elapsed().as_secs_f32());
    };

    let mainfut = async {
        fut1.await;
        fut2.await;
    };

    block_on(mainfut);
}

use std::{
    future::Future, sync::{ mpsc::{channel, Sender}, Arc, Mutex, Condvar},
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker}, mem, pin::Pin,
    thread::{self, JoinHandle}, time::{Duration, Instant}, collections::HashMap
};
// ============================= EXECUTOR ====================================
#[derive(Default)]
struct Parker(Mutex<bool>, Condvar);

impl Parker {
    fn park(&self) {
        let mut resumable = self.0.lock().unwrap();
            while !*resumable {
                resumable = self.1.wait(resumable).unwrap();
            }
        *resumable = false;
    }

    fn unpark(&self) {
        *self.0.lock().unwrap() = true;
        self.1.notify_one();
    }
}

fn block_on<F: Future>(mut future: F) -> F::Output {
    let parker = Arc::new(Parker::default());
    let mywaker = Arc::new(MyWaker { parker: parker.clone() });
    let waker = mywaker_into_waker(Arc::into_raw(mywaker));
    let mut cx = Context::from_waker(&waker);

    // SAFETY: we shadow `future` so it can't be accessed again.
    let mut future = unsafe { Pin::new_unchecked(&mut future) };
    loop {
        match Future::poll(future.as_mut(), &mut cx) {
            Poll::Ready(val) => break val,
            Poll::Pending => parker.park(),
        };
    }
}
// ====================== FUTURE IMPLEMENTATION ==============================
#[derive(Clone)]
struct MyWaker {
    parker: Arc<Parker>,
}
#[derive(Clone)]
pub struct Task {
    id: usize,
    reactor: Arc<Mutex<Box<Reactor>>>,
    data: u64,
}

fn mywaker_wake(s: &MyWaker) {
    let waker_arc = unsafe { Arc::from_raw(s) };
    waker_arc.parker.unpark();
}

fn mywaker_clone(s: &MyWaker) -> RawWaker {
    let arc = unsafe { Arc::from_raw(s) };
    std::mem::forget(arc.clone()); // increase ref count
    RawWaker::new(Arc::into_raw(arc) as *const (), &VTABLE)
}

const VTABLE: RawWakerVTable = unsafe {
    RawWakerVTable::new(
        |s| mywaker_clone(&*(s as *const MyWaker)),   // clone
        |s| mywaker_wake(&*(s as *const MyWaker)),    // wake
        |s| (*(s as *const MyWaker)).parker.unpark(), // wake by ref
        |s| drop(Arc::from_raw(s as *const MyWaker)), // decrease refcount
    )
};

fn mywaker_into_waker(s: *const MyWaker) -> Waker {
    let raw_waker = RawWaker::new(s as *const (), &VTABLE);
    unsafe { Waker::from_raw(raw_waker) }
}

impl Task {
    fn new(reactor: Arc<Mutex<Box<Reactor>>>, data: u64, id: usize) -> Self {
        Task { id, reactor, data }
    }
}

impl Future for Task {
    type Output = usize;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut r = self.reactor.lock().unwrap();
        if r.is_ready(self.id) {
            *r.tasks.get_mut(&self.id).unwrap() = TaskState::Finished;
            Poll::Ready(self.id)
        } else if r.tasks.contains_key(&self.id) {
            r.tasks.insert(self.id, TaskState::NotReady(cx.waker().clone()));
            Poll::Pending
        } else {
            r.register(self.data, cx.waker().clone(), self.id);
            Poll::Pending
        }
    }
}
// =============================== REACTOR ===================================
enum TaskState {
    Ready,
    NotReady(Waker),
    Finished,
}
struct Reactor {
    dispatcher: Sender<Event>,
    handle: Option<JoinHandle<()>>,
    tasks: HashMap<usize, TaskState>,
}

#[derive(Debug)]
enum Event {
    Close,
    Timeout(u64, usize),
}

impl Reactor {
    fn new() -> Arc<Mutex<Box<Self>>> {
        let (tx, rx) = channel::<Event>();
        let reactor = Arc::new(Mutex::new(Box::new(Reactor {
            dispatcher: tx,
            handle: None,
            tasks: HashMap::new(),
        })));

        let reactor_clone = Arc::downgrade(&reactor);
        let handle = thread::spawn(move || {
            let mut handles = vec![];
            for event in rx {
                let reactor = reactor_clone.clone();
                match event {
                    Event::Close => break,
                    Event::Timeout(duration, id) => {
                        let event_handle = thread::spawn(move || {
                            thread::sleep(Duration::from_secs(duration));
                            let reactor = reactor.upgrade().unwrap();
                            reactor.lock().map(|mut r| r.wake(id)).unwrap();
                        });
                        handles.push(event_handle);
                    }
                }
            }
            handles.into_iter().for_each(|handle| handle.join().unwrap());
        });
        reactor.lock().map(|mut r| r.handle = Some(handle)).unwrap();
        reactor
    }

    fn wake(&mut self, id: usize) {
        let state = self.tasks.get_mut(&id).unwrap();
        match mem::replace(state, TaskState::Ready) {
            TaskState::NotReady(waker) => waker.wake(),
            TaskState::Finished => panic!("Called 'wake' twice on task: {}", id),
            _ => unreachable!()
        }
    }

    fn register(&mut self, duration: u64, waker: Waker, id: usize) {
        if self.tasks.insert(id, TaskState::NotReady(waker)).is_some() {
            panic!("Tried to insert a task with id: '{}', twice!", id);
        }
        self.dispatcher.send(Event::Timeout(duration, id)).unwrap();
    }

    fn is_ready(&self, id: usize) -> bool {
        self.tasks.get(&id).map(|state| match state {
            TaskState::Ready => true,
            _ => false,
        }).unwrap_or(false)
    }
}

impl Drop for Reactor {
    fn drop(&mut self) {
        self.dispatcher.send(Event::Close).unwrap();
        self.handle.take().map(|h| h.join().unwrap()).unwrap();
    }
}
*/


/*
// https://www.heise.de/hintergrund/Ferris-Talk-4-Asynchrone-Programmierung-in-Rust-6299096.html?seite=2
use std::{pin::Pin, task::{Context, Poll}, thread, time::{Duration, Instant}};
use std::future::Future;    // or futures::Future, both works

struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>)
        -> Poll<&'static str> {
        if Instant::now() >= self.when {
            Poll::Ready("done")
        } else {
            let waker = cx.waker().clone();
            let when = self.when;

           thread::spawn(move || {
                let now = Instant::now();

                if now < when {
                    thread::sleep(when - now);
                }
                waker.wake();
            });
            Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {

    let main_time: tokio::time::Instant = tokio::time::Instant::now();

    let when = Instant::now() + Duration::from_millis(100);
    let future = Delay { when };

    let out = future.await;
    println!("{}", out);
    assert_eq!(out, "done");

    // End of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\n⌛️ Execution time: {:?} ({:?} ms)", duration, elapsed_ms);
}
*/



/*
// https://users.rust-lang.org/t/multithreading-with-multiple-writes-to-same-slice-without-lock/52415/11
// https://stackoverflow.com/questions/65178245/how-do-i-write-to-a-mutable-slice-from-multiple-threads-at-arbitrary-indexes-wit
use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;

struct UnsafeContainer<T> {
    data: UnsafeCell<T>,
}

impl<T> UnsafeContainer<T> {
    fn new(data: T) -> UnsafeContainer<T> {
        UnsafeContainer {
            data: UnsafeCell::new(data),
        }
    }
}

unsafe impl<T> Sync for UnsafeContainer<T> {}
unsafe impl<T> Send for UnsafeContainer<T> {}

pub fn caller(arr: &mut [i32]) {
    let mut buf: Vec<i32> = vec![0; arr.len()];
    do_something(&mut arr[..arr.len()/2], &mut arr[arr.len()/2..], &mut buf[..], 8);
}

fn do_something(arr1: &[i32], arr2: &[i32], arr3: &'static mut [i32], num_cores: usize) {
    let mut children = Vec::new();

    let shared = Arc::new(UnsafeContainer::new(arr3));

    for i in 0..num_cores {
        let container = shared.clone();
        children.push(thread::spawn(move || unsafe {
            println!("{:?}", container.data.get());
            // I really want to read from one of the arrays here and write to arr3
        }));
    }

    for child in children {
        child.join();
    }
}

fn main() {}
*/


/*
// alice suggestion
use std::cell::UnsafeCell;

#[allow(unused)]
#[derive(Copy, Clone)]
struct UnsafeSlice<'a, T> {
    slice: &'a [UnsafeCell<T>],
}
unsafe impl<'a, T: Send + Sync> Send for UnsafeSlice<'a, T> {}
unsafe impl<'a, T: Send + Sync> Sync for UnsafeSlice<'a, T> {}


impl<'a, T> UnsafeSlice<'a, T> {
	
	#[allow(unused)]

    pub fn new(slice: &'a mut [T]) -> Self {
        let ptr = slice as *mut [T] as *const [UnsafeCell<T>];
        Self {
            slice: unsafe { &*ptr },
        }
    }
    
    /// SAFETY: It is UB if two threads write to the same index without
    /// synchronization.
    pub unsafe fn write(&self, i: usize, value: T) {
        let ptr = self.slice[i].get();
        *ptr = value;
    }
}

fn main() {
	println!("it works!");
}
*/


/*
// Problem: error[E0507]: cannot move out of self as enum variant Baz which is behind a mutable reference

use std::ptr;

#[allow(unused)]
enum Foo<T> {
    Bar(T),
    Baz(T),
}

impl<T> Foo<T> {
	
	#[allow(unused)]
	
	// wont work 
    fn switch(&mut self) {
        *self = match self {
            &mut Foo::Bar(val) => Foo::Baz(val),
            &mut Foo::Baz(val) => Foo::Bar(val),
        }
    }
	
	
	// works
	fn switch(&mut self) {
        // I copied this code from Stack Overflow without reading
        // the surrounding text that explains why this is safe.
        unsafe {
            let tmp = ptr::read(self);
    
            // Must not panic before we get to ptr::write

            let new = match tmp {
                Foo::Bar(val) => Foo::Baz(val),
                Foo::Baz(val) => Foo::Bar(val),
            };
    
            ptr::write(self, new);
        }
    }
	
}

fn main() {
	println!("it works!");
}
*/



/*
use std::ptr;

// Note: This definition is naive. See the chapter on implementing Vec.
pub struct Vec<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
}

// Note this implementation does not correctly handle zero-sized types.
// See the chapter on implementing Vec.
impl<T> Vec<T> {
    pub fn push(&mut self, elem: T) {
        if self.len == self.cap {
            // not important for this example
            //self.reallocate();
        }
        //unsafe {
            ptr::write(self.ptr.add(self.len), elem);
            self.len += 1;
        //}
    }
}

fn main() {
	println!("it works!");
}
*/



// try to get a borrow checker error from: https://users.rust-lang.org/t/unsafe-rust-and-the-borrow-checker-multiple-mutable-borrows/63293

/* if remove unsafe: error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
   --> compiler.rs:298:20
    |
298 |         pointer2 = &mut *(pointer as *mut i32);
    |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^ dereference of raw pointer
    |
    = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
*/

/*
fn main() {
    let mut i1 = 1;
    let pointer = &mut i1;
    let pointer2: &mut i32;
    //unsafe {
        pointer2 = &mut *(pointer as *mut i32);
    //}
    println!("Set *pointer2 = 2");
    *pointer2 = 2;
    println!("==> Value of {:<8} is {}","pointer", pointer); // Outputs 2
    println!("==> Value of {:<8} is {}","pointer2", pointer2); // Outputs 2
    println!("Set *pointer = 5");
    *pointer = 5;
    println!("==> Value of {:<8} is {}", "pointer", pointer); // Outputs 5
    println!("==> Value of {:<8} is {}", "pointer2", pointer2); // Outputs 5
}

//fn main() {println!("it works!");}
*/




// problem: "value moved here, in previous iteration of the loop"




/*
// deadlock situation:
use std::sync::{Arc, Mutex, MutexGuard};

fn main() {
    let vector: Arc<Mutex<Vec<u64>>> = Arc::new(Mutex::new(Vec::new()));
	let mut locked_vector: MutexGuard<'_, Vec<u64>> = vector.lock().unwrap();
	locked_vector.push(2);
    locked_vector.push(3);

    for i in locked_vector.iter() { // took forever to run in mobile ! => infinite loop run
		println!("{}", i);
    }
    println!("No more item left in vector");


    std::thread::sleep(std::time::Duration::from_millis(200));

    read_vector(&vector);
}


fn read_vector(p: &Mutex<Vec<u64>>) {
	let p_guard = p.lock().unwrap();
	for i in p_guard.iter() { // took forever to run in mobile ! => infinite loop run
		println!("{}", i);
    }
}
*/


/*
// ultimate deadlock & resolve example
use std::sync::{Arc, Mutex};

fn main() {
    let vector = Arc::new(Mutex::new(Vec::new()));

    // read_vector(&vector);   // -> fine, cuz it does not take ownershio or hold lock
    // let locked_vector: std::sync::MutexGuard<'_, Vec<u64>> = vector.lock().unwrap(); // -> deadlock cuz not release lock yet

    {
        println!("[from lexial scope]");
        let mut locked_vector: std::sync::MutexGuard<'_, Vec<u64>> = vector.lock().unwrap();
        locked_vector.push(2);
        locked_vector.push(3);

        for i in locked_vector.iter() {
            println!("{}", i);
        }
        println!("No more items left in vector");
    }   // MutexGuard dropped here

    // Using a closure to release/drop the lock explicitly
    {
        println!("[from closure]");
        let vector_clone: Arc<Mutex<Vec<u64>>> = vector.clone(); // Clone Arc for closure
        std::thread::spawn(move || {
            read_vector(&vector_clone);
        }).join().expect("Thread panicked");
    }

}

fn read_vector(mutex_vec: &Arc<Mutex<Vec<u64>>>) {
    let mutex_vec_guard: std::sync::MutexGuard<'_, Vec<u64>> = mutex_vec.lock().unwrap();
    for i in mutex_vec_guard.iter() {
        println!("{}", i);
    }
    println!("No more items left in vector");
}
*/




/*
// hỏi moved mà đưa code lỗi operation
// Problem: value moved here, in previous iteration of loop
// https://users.rust-lang.org/t/how-to-solve-this-used-of-moved-value-issue/68138

pub fn test(mut a: Vec<u8>) -> (Vec<u8>, i32) {
    let mut b  = Vec::new();
    let mut c = 0;
    loop {
        if a.len() == 0 { break; }
        let d = a.pop().unwrap();
        c = c+ (d as i32);
        b.push(d);
    }
    (b, c)
}


#[allow(unused)] // supress all warns
fn borrow_test() {

    let mut a = vec![1,2,3,4,5];
    let mut i = 0;
    let c = 0;
    loop {
        let (a, c) = test(a);
        println!("{}",c);
        if i >=5 {break;}
    }
}

fn main() {
	borrow_test();
}
*/



/*
// deadlock cycle 2 resources & 2 threads
use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    // Define two resources
    let resource1 = Arc::new(Mutex::new(()));
    let resource2 = Arc::new(Mutex::new(()));

    // Clone the resources for use in the threads
    let resource1_clone = Arc::clone(&resource1);
    let resource2_clone = Arc::clone(&resource2);

    // Thread 1
    let thread1 = thread::spawn(move || {
        let _lock1 = resource1_clone.lock().unwrap();
        println!("Thread 1 has acquired resource 1");

        // Simulate some work
        thread::sleep(std::time::Duration::from_secs(1));

        let _lock2 = resource2_clone.lock().unwrap();
        println!("Thread 1 has acquired resource 2");
    });

    // Thread 2
    let thread2 = thread::spawn(move || {
        let _lock2 = resource2.lock().unwrap();
        println!("Thread 2 has acquired resource 2");

        // Simulate some work
        thread::sleep(std::time::Duration::from_secs(1));

        let _lock1 = resource1.lock().unwrap();
        println!("Thread 2 has acquired resource 1");
    });

    // Wait for both threads to finish
    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("Both threads have finished");
}
*/
/*
Output:
Thread 1 has acquired resource 1
Thread 2 has acquired resource 2        <-- deadlock !
*/



/*
// solve deadlock by ensure that threads acquire resources in a consistent order to prevent cyclic dependencies.
use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;

fn main() {
    let main_time: std::time::Instant = std::time::Instant::now();

    // Define two resources
    let resource1 = Arc::new(Mutex::new(()));
    let resource2 = Arc::new(Mutex::new(()));

    // Clone the resources for use in the threads
    let resource1_clone = Arc::clone(&resource1);
    let resource2_clone = Arc::clone(&resource2);

    // Thread 1
    let thread1 = thread::spawn(move || {
        // Attempt to acquire resource 1
        let lock1 = acquire_lock_with_retry(&resource1_clone, &resource2_clone);
        if let Ok(_) = lock1 {
            println!("Thread 1 has acquired resource 1");

            // Simulate some work
            // thread::sleep(Duration::from_secs(1));

            // Acquire resource 2
            let _lock2 = resource2_clone.lock().unwrap();
            println!("Thread 1 has acquired resource 2");
        } else {
            println!("Thread 1 failed to acquire resources, deadlock detected");
        }
    });

    // Thread 2
    let thread2 = thread::spawn(move || {
        // Attempt to acquire resource 1
        let lock1 = acquire_lock_with_retry(&resource1, &resource2);
        if let Ok(_) = lock1 {
            println!("Thread 2 has acquired resource 1");

            // Simulate some work
            // thread::sleep(Duration::from_secs(1));

            // Acquire resource 2
            let _lock2 = resource2.lock().unwrap();
            println!("Thread 2 has acquired resource 2");
        } else {
            println!("Thread 2 failed to acquire resources, deadlock detected");
        }
    });

    // Wait for both threads to finish
    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("Both threads have finished");

    // end of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\n⌛️ Execution time: {:?} ({:?} ms)", duration, elapsed_ms);
}

fn acquire_lock_with_retry(resource1: &Arc<Mutex<()>>, resource2: &Arc<Mutex<()>>) -> Result<(), String> {
    loop {
        // Attempt to acquire resource 1
        let lock1 = resource1.lock();
        if lock1.is_err() {
            // Retry after a short delay
            thread::sleep(Duration::from_millis(20));
            continue;
        }

        // Attempt to acquire resource 2
        let lock2 = resource2.lock();
        if lock2.is_err() {
            // Release resource 1 and retry after a short delay
            drop(lock1.unwrap());
            thread::sleep(Duration::from_millis(20));
            continue;
        }

        // Both resources acquired successfully
        return Ok(());
    }
}
*/
/*
In this updated code:
Each thread attempts to acquire resources in a fixed order (first resource1, then resource2).
If a thread fails to acquire a resource, it releases any previously acquired resources and retries after a short delay.
This approach ensures that threads do not enter into a cyclic dependency where they are waiting for resources held by each other, effectively preventing deadlocks.
With these changes, the code should no longer deadlock, and both threads should complete their execution successfully.
*/
/* 
Output:
Thread 1 has acquired resource 1
Thread 1 has acquired resource 2
Thread 2 has acquired resource 1
Thread 2 has acquired resource 2
Both threads have finished

⌛️ Execution time: 236.521µs (0.236521 ms)
*/



/*
// avoiding clonning approach. In this case, both threads acquire their initial resources successfully but then wait indefinitely when attempting to acquire the other resource, resulting in a deadlock.
use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;

fn main() {
    // Define two resources
    let resource1 = Arc::new(Mutex::new(()));
    let resource2 = Arc::new(Mutex::new(()));

    // Spawn threads
    let thread1 = spawn_thread("Thread 1", &resource1, &resource2);
    let thread2 = spawn_thread("Thread 2", &resource2, &resource1);

    // Wait for threads to finish
    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("Both threads have finished");
}

fn spawn_thread(name: &'static str, this: &Arc<Mutex<()>>, other: &Arc<Mutex<()>>) -> thread::JoinHandle<()> {
    let this_clone = Arc::clone(this);
    let other_clone = Arc::clone(other);

    thread::spawn(move || {
        // Acquire locks
        let this_lock = this_clone.lock().unwrap();
        println!("{} has acquired its resource", name);

        // Simulate work
        thread::sleep(Duration::from_secs(1));

        // Attempt to acquire the other resource
        let other_lock = other_clone.lock();
        match other_lock {
            Ok(_) => println!("{} has acquired the other resource", name),
            Err(_) => println!("{} failed to acquire the other resource", name),
        }
    })
}
*/


/*
// fixed deadlock:
use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;

fn main() {
    let main_time: std::time::Instant = std::time::Instant::now();

    // Define two resources
    let resource1 = Arc::new(Mutex::new(()));
    let resource2 = Arc::new(Mutex::new(()));

    // Spawn threads
    let thread1 = spawn_thread("Thread 1", &resource1, &resource2);
    let thread2 = spawn_thread("Thread 2", &resource1, &resource2);

    // Wait for threads to finish
    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("Both threads have finished");

    // end of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\n⌛️ Execution time: {:?} ({:?} ms)", duration, elapsed_ms);
}

fn spawn_thread(name: &'static str, this: &Arc<Mutex<()>>, other: &Arc<Mutex<()>>) -> thread::JoinHandle<()> {
    let this_clone = Arc::clone(this);
    let other_clone = Arc::clone(other);

    thread::spawn(move || {
        // Acquire locks in a fixed order
        let this_lock = this_clone.lock().unwrap();
        println!("{} has acquired its resource", name);

        // Simulate work
        // thread::sleep(Duration::from_secs(1));

        // Acquire the other resource
        let other_lock = other_clone.lock();
        match other_lock {
            Ok(_) => println!("{} has acquired the other resource", name),
            Err(_) => println!("{} failed to acquire the other resource", name),
        }
    })
}
*/
/*
Output: (not much improvement in performant than previous approach (~30µs reduction ))
Thread 1 has acquired its resource
Thread 1 has acquired the other resource
Thread 2 has acquired its resource
Thread 2 has acquired the other resource
Both threads have finished

⌛️ Execution time: 192.838µs (0.192838 ms)
*/
/*
In this fix:
 1. Both threads now attempt to acquire resources in the same fixed order (resource1 first, then resource2).
 2. By ensuring a consistent order for acquiring resources, we prevent cyclic dependencies and potential deadlocks.
*/





// fn main() {
//     // Rust Closure Function
//     let add_numbers = |a: i32, b: i32| -> i32 {
//         a + b
//     };

//     // Usage
//     let result = add_numbers(3, 5);
//     println!("Result: {}", result); 
// }
// // Output: Result: 8


// fn main() {
//     let main_time: std::time::Instant = std::time::Instant::now();

//     println!("this program measure time taken for its to execute.");

//     // end of main
//     let duration: std::time::Duration = main_time.elapsed();
//     let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
//     let elapsed_micro: f64 = elapsed_ms * 1000.0;
//     let elapsed_sec: f64 = elapsed_ms / 1000.0;
//     println!("\n⌛️ Execution time: {:?} ({:.8} ms) ({:.6} µs) ({:.8} s)", duration, elapsed_ms, elapsed_micro, elapsed_sec);
// }
/*
Output:
this program measure time taken for its to execute.

⌛️ Execution time: 31.678µs (0.03167800 ms) (31.678000 µs) (0.00003168 s)
*/



/*
// https://stackoverflow.com/questions/59156473/what-is-the-difference-between-async-move-and-async-move
use futures::executor::block_on;
use futures::future::{FutureExt, TryFutureExt};


async fn fut1() -> Result<String, u32> {
    Ok("ok".to_string())
}

fn main() {
    let main_time: std::time::Instant = std::time::Instant::now();

    println!("Hello, world!");
    match block_on(fut1().and_then(|x| async move { Ok(format!("{} is \"ok\"", x)) })) {
      Ok(s) => println!("{}", s),
      Err(u) => println!("{}", u)
    };

    // end of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    let elapsed_micro: f64 = elapsed_ms * 1000.0;
    let elapsed_sec: f64 = elapsed_ms / 1000.0;
    println!("\n⌛️ Execution time: {:?} ({:.8} ms) ({:.6} µs) ({:.8} s)", duration, elapsed_ms, elapsed_micro, elapsed_sec);

}
*/



/*
// https://stackoverflow.com/questions/74230861/async-move-closure-vs-fold?noredirect=1&lq=1
use futures::{stream, StreamExt};
use rand::{thread_rng, Rng};
use std::time::Duration;

async fn compute(i: i32) -> i32 {
    let mut rng = thread_rng();
    // let sleep_ms: u64 = rng.gen_range(0..1000);
    // tokio::time::sleep(Duration::from_millis(sleep_ms)).await;
    println!("#{} done", i);
    i * i
}

async fn sum_with_fold() {
    let sum = stream::iter(1..25)
        .map(compute)
        .buffered(12)
        .fold(0, |sum,x| async move {sum+x} )
        .await;
    println!("->{}", sum);
}

// wrong code
// async fn sum_with_closure() {
//     let mut sum: i32 = 0;
//     stream::iter(1..25)
//         .map(compute)
//         .buffered(12)
//         .for_each(|result| async move { sum+=result; })
//         .await;
//     println!("->{}", sum);
// }

// correct code: This can be done with for_each, but current Rust can't check at compile-time that the lifetimes are correct so you need to use a RefCell to enable run-time checking (with a small performance cost):
async fn sum_with_closure() {
    use std::cell::RefCell;     // <--

    let sum = RefCell::new (0);
    let sumref = &sum;
    stream::iter(1..25)
        .map(compute)
        .buffered(12)
        .for_each(|result| async move { *sumref.borrow_mut() +=result; })
        .await;
    println!("->{}", sum.into_inner());
}

#[tokio::main]
async fn main() {

    let sum_with_fold_time: std::time::Instant = std::time::Instant::now();
    sum_with_fold().await;

    // end of sum_with_fold()
    let duration: std::time::Duration = sum_with_fold_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    let elapsed_micro: f64 = elapsed_ms * 1000.0;
    let elapsed_sec: f64 = elapsed_ms / 1000.0;
    println!("\n⌛️ Fold Execution time: {:?} ({:.8} ms) ({:.6} µs) ({:.8} s)", duration, elapsed_ms, elapsed_micro, elapsed_sec);

    let sum_with_closure_time: std::time::Instant = std::time::Instant::now();
    sum_with_closure().await;

    // end of sum_with_closure()
    let duration: std::time::Duration = sum_with_closure_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    let elapsed_micro: f64 = elapsed_ms * 1000.0;
    let elapsed_sec: f64 = elapsed_ms / 1000.0;
    println!("\n⌛️ Closure Execution time: {:?} ({:.8} ms) ({:.6} µs) ({:.8} s)", duration, elapsed_ms, elapsed_micro, elapsed_sec);
}
*/


/*
// https://stackoverflow.com/questions/30177395/when-does-a-closure-implement-fn-fnmut-and-fnonce
// Fn trait:
fn main() {
    let some_vec = vec![1, 3, 4];
    
    let get_that_same_vec = || { // "get_that_same_vec" type: impl Fn() -> &Vec<i32>
        &some_vec
        // as you can see the closure is specified to implement the *Fn* trait,
        // meaning it can be called however many times since it doesn't alter the data it operates on
        // or pass the ownership of that data to any other entity 
    };
    // - By using the "&" above we are basically saying that a closure  should just return a reference to a value.
    // - If we were to omit the "&" above, we would basically be saying:
    // "return and pass the ownership of this value to whomever decides to invoke this closure"
    // which would basically be like turning it into an infinite generator of that value and its ownership
    // "Oh, another caller wants this value and the ownership of it? Sure, let me take the copy of that value... out of thin air I guess!"
    // As you can figure, Rust doesn't allow for that last sentence to be true,
    // since that would allow multiple entities to have the ownership of the underlying memory,
    // which would eventually result in a "double free" error when needing to free that underlying memory when one of the owners goes out of scope. (see: *FnOnce* example)   

    println!("This is the vec: {:?}", get_that_same_vec());
    println!("This is the vec: {:?}", get_that_same_vec()); // if "&" would not be present above, then this would not compile
}
*/



/*
// https://stackoverflow.com/questions/26577070/how-to-use-the-fn-traits-closures-in-rust-function-signatures?rq=3
fn closures<F1, F2, F3>(mut f1: F1, mut f2: F2, mut f3: F3) -> i32
where
    F1: FnMut() -> f32,
    F2: FnMut(i32) -> f32,
    F3: FnMut(i32, i32) -> f32,
{
    (f1() + f2(10) + f3(20, 30)) as i32
}

fn main() {
    let x = closures(|| 0.1, |x| (2 * x) as f32, |x, y| (x + y) as f32);
    println!("{}", x);
}
*/



/*
// https://stackoverflow.com/questions/16421033/lazy-sequence-generation-in-rust
struct MyRange {
    start: u64,
    end: u64,
}

impl MyRange {
    fn new(start: u64, end: u64) -> MyRange {
        MyRange {
            start: start,
            end: end,
        }
    }
}

impl Iterator for MyRange {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.start == self.end {
            None
        } else {
            let result = Some(self.start);
            self.start += 1;
            result
        }
    }
}

fn main() {
    let sum: u64 = MyRange::new(0, 1_000_000).sum();
    println!("{}", sum)
}
*/



/*
// https://stackoverflow.com/questions/70218626/how-do-i-use-the-yield-keyword-in-rust?noredirect=1&lq=1
// An iterator needs to implement Iterator trait.
// Rust doesn't use of the yield keyword for generators (for now, Rust 1.57),
// so you cannot use it. Direct translation of your code would be:
fn func() -> impl Iterator<Item=u32> {
    0..100u32
}

fn main() {
    for i in func() {
        println!("{}", i);
    }
}
*/



/*
// https://stackoverflow.com/questions/45985827/how-can-i-replace-the-value-inside-a-mutex?rq=3
// You can assign a new value to the reference using the dereference operator *.
// If you need the previous value, you can use std::mem::replace
use std::sync::Mutex;
use std::mem;

fn example_not_using_old_value(state: &Mutex<String>) {
    let mut state = state.lock().expect("Could not lock mutex");
    *state = String::from("dereferenced");
}

fn example_using_old_value(state: &Mutex<String>) -> String {
    let mut state = state.lock().expect("Could not lock mutex");
    mem::replace(&mut *state, String::from("replaced"))
}

fn main() {
    let state = Mutex::new("original".into());
    example_not_using_old_value(&state);
    let was = example_using_old_value(&state);

    println!("Is now {:?}", state);
    println!("Was {:?}", was);
}
*/



/*
// answered in 2015 by Shepmaster
// https://stackoverflow.com/questions/28258548/cannot-move-out-of-borrowed-content-when-trying-to-transfer-ownership?rq=3
// Cannot move out of borrowed content when trying to transfer ownership
// At a high-level, this is against-the-grain for Rust. You cannot transfer ownership of something borrowed because you don't own it.
pub struct LinkedList {
    head: Option<Box<LinkedListNode>>,
}

pub struct LinkedListNode {
    next: Option<Box<LinkedListNode>>,
}

impl LinkedList {
    pub fn new() -> LinkedList {
        LinkedList { head: None }
    }

    // won't compile
    // pub fn prepend_value(&mut self) {
    //     let mut new_node = LinkedListNode { next: None };

    //     match self.head {
    //         Some(ref head) => new_node.next = Some(*head),
    //         None => new_node.next = None,
    //     };

    //     self.head = Some(Box::new(new_node));
    // }


    // In this case, you can use `Option::take`.
    // This will leave the variable where it is, changing it in-place 
    // to a None and returning the previous value. You can then use
    // that value to build the new head of the list:
    pub fn prepend_value(&mut self) {
        let head = self.head.take();
        self.head = Some(Box::new(LinkedListNode { next: head }));
    }
}

fn main() {}
*/



/* 
// anwsered by Shepmaster
// (use of partially moved value: `self`)
// https://stackoverflow.com/questions/40615054/reference-to-unwrapped-property-fails-use-of-partially-moved-value-self?noredirect=1&lq=1
fn main() {
    let a = A {p: Some("p".to_string())};
    a.a();
}

struct A {
    p: Option<String>
}

impl A {
    fn a(self) -> Self {
        self.b();
        self
    }

    fn b(&self) {
        print!("b: {}", self.p.as_ref().unwrap())
    }
}
*/




/*
// udp
use tokio::net::UdpSocket;
use tokio::select;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let sock1 = UdpSocket::bind("127.0.0.1:8080").await?;
    let sock2 = UdpSocket::bind("127.0.0.1:7070").await?;
    let mut buf1=vec![0;1024];
    let mut buf2=vec![0;1024];
    loop {
        let (len, addr, socket_id)=select!{
            Ok((len, addr))=sock1.recv_from(&mut buf1)=>{(len, addr, 0)}
            Ok((len, addr))=sock2.recv_from(&mut buf2)=>{(len, addr, 1)}
        };
        println!("{} {} {}", len, addr, socket_id);
        //buf1/2 are sent through sender[socket_id]
    }
}
*/




/*
use futures::executor::block_on;
use futures::future::{FutureExt, TryFutureExt};

async fn fut1() -> Result<String, u32> {
    Ok("ok".to_string())
}

fn main() {
    println!("Hello, world!");
    match block_on(fut1().and_then(async move |x| { Ok(format!("{} is \"ok\"", x)) })) {
      Ok(s) => println!("{}", s),
      Err(u) => println!("{}", u)
    };
}
*/





/*
use std::time::{Duration, Instant};
use tokio::time::sleep;
use tokio::spawn;
use rand::Rng;

#[tokio::main]
async fn main() {
    let mut handles = Vec::new();
    for i in 1..10 {
        let millis = rand::thread_rng().gen_range(1..200);
        let start = Instant::now();
        let r = spawn(async move {
            println!("hi number {} from the spawned thread!", i);
            // sleep(Duration::from_millis(millis)).await;
            // let duration = start.elapsed();
            // println!("bye number {} from the spawned thread! Duration: {:?}", i, duration);
            println!("bye number {} from the spawned thread!", i);
        });
        handles.push(r);
    }

    for i in handles {
        i.await.unwrap();
    }
}
*/




/*
// https://users.rust-lang.org/t/how-can-i-make-tokio-spawn-tasks-at-the-same-time/39589
use tokio::{io::{AsyncReadExt, BufReader}, net::TcpListener, time::Instant};
use console;

#[tokio::main]
async fn main() {
    let mut listener = TcpListener::bind("127.0.0.1:2020").await.unwrap();

    println!("[ {} ] Waiting for files..", console::style("START").green().bold());

    use std::sync::{mpsc, atomic::{AtomicU32, Ordering}, Arc};

    let (send, recv) = mpsc::channel();
    tokio::spawn(async move {
        let mut counters: Vec<Arc<AtomicU32>> = vec![];
        const TOTAL_LEN: u32 = 20000000;
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            while let Ok(counter) = recv.try_recv() {
                counters.push(counter);
            }
            for counter in &counters {
                print!("{:>3} ", counter.load(Ordering::Relaxed) * 100 / TOTAL_LEN);
            }
            // println!();
        }
    });

    loop {
        let (stream, addr) = listener.accept().await.unwrap();
        let now = Instant::now();

        println!(
            "[  {}  ] Processing file from {}",
            console::style("NEW").green(),
            console::style(addr).yellow()
        );

        let counter = Arc::new(AtomicU32::new(0));
        send.send(counter.clone()).unwrap();

        tokio::spawn(async move {
            let mut reader = BufReader::new(stream);
            let mut buf = [0u8; 1024];
            let mut _nb_alphas = 0;

            while let Ok(len) = reader.read(&mut buf).await {
                if len == 0 { break }

                for b in &buf[..len] {
                    if matches!(*b, b'a'..=b'z' | b'A'..=b'Z') {
                        _nb_alphas += 1;
                    }
                }

                counter.fetch_add(len as u32, Ordering::Relaxed);

                tokio::task::yield_now().await;
            }

            // time!(now, addr);
            println!("{addr}");
        });
    }
}
*/




/*
// https://rust-lang.github.io/async-book/03_async_await/01_chapter.html
use futures::Future;

/// `async` block:
///
/// Multiple different `async` blocks can access the same local variable
/// so long as they're executed within the variable's scope
async fn blocks() {
    let my_string = "foo".to_string();

    let future_one = async {
        // ...
        println!("{my_string}");
    };

    let future_two = async {
        // ...
        println!("{my_string}");
    };

    // Run both futures to completion, printing "foo" twice:
    let ((), ()) = futures::join!(future_one, future_two);
}

/// `async move` block:
///
/// Only one `async move` block can access the same captured variable, since
/// captures are moved into the `Future` generated by the `async move` block.
/// However, this allows the `Future` to outlive the original scope of the
/// variable:
fn move_block() -> impl Future<Output = ()> {
    let my_string = "foo".to_string();
    async move {
        // ...
        println!("{my_string}");
    }
}

fn main() {
    
}
*/



// this topic consists a response that has 5 references to learn
// https://stackoverflow.com/questions/60717746/how-to-accept-an-async-function-as-an-argument?noredirect=1&lq=1




/*
// async_printing.rs
use rand::seq::SliceRandom;
use std::time::Duration;
use tokio::time::sleep;
use tokio::runtime::Runtime;
use std::pin::Pin;
use std::future::Future;

async fn print_random_text() {
    let random_texts = [
        "Hello", "This", "Is", "An", "Example", "Of", "Printing", "Random", "Text", "One", "By", "One",
    ];

    for text in random_texts.choose_multiple(&mut rand::thread_rng(), random_texts.len()) {
        println!("{}", text);
        sleep(Duration::from_millis(100)).await;
    }
}

fn get_my_pods() -> impl Future<Output=()> {
    async {
        // Simulate an asynchronous operation
        sleep(Duration::from_secs(1)).await;
        println!("Get all my pods in default namespace");
    }
}

//#[tokio::main]
fn main() {
    //print_random_text().await;

    // create a runtime, this make main() no longer async method
    // we execute async tasks inside this runtime
    let mut rt = Runtime::new().unwrap();
    rt.block_on(async {
        print_random_text().await;
        get_my_pods().await;
        println!("all done!");
    });
}
*/




// rust inheritance
/*
trait Animal {
    fn speak(&self);
    fn custom_behavior(&self) {
        println!("Default behavior for Animal");
    }
}

struct Dog {
    name: String,
    breed: String,
}

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }

    fn custom_behavior(&self) {
        println!("Custom behavior for Dog");
    }
}

struct Cat {
    name: String,
    breed: String,
}

impl Animal for Cat {
    fn speak(&self) {
        println!("Meow!");
    }

    fn custom_behavior(&self) {
        println!("Custom behavior for Cat");
    }
}

fn main() {
    let dog = Dog {
        name: String::from("Buddy"),
        breed: String::from("Golden Retriever"),
    };

    dog.speak();
    dog.custom_behavior(); // This will call the overridden custom_behavior method for Dog

    let cat = Cat {
        name: String::from("Whiskers"),
        breed: String::from("Siamese"),
    };

    cat.speak();

    cat.custom_behavior(); // This will call the overridden custom_behavior method for Cat
}
*/




// https://users.rust-lang.org/t/polymorphism-in-rust-is-it-possible/81691/2
/*
use std::env;

trait Animal {

    // &self doesn't take ownership or move data; instead, it borrows the data. 
    fn get_name(&self) -> String {
        "no name".to_owned()
    }
    fn get_age(&self) -> i32;
}

#[derive(Clone)]
struct Human {
    pub name: String,
    pub age: i32,
}

impl Human {
    fn new(name: String, age: i32) -> Human {
        Human { name, age }
    }
}

impl Animal for Human {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_age(&self) -> i32 {
        self.age
    }
}

trait Pet: Animal {
    fn show_affection(&self) {
        println!("I affect every Pet instance.");
    }
}

#[derive(Clone)]
struct Dog {
    title: String,
    old: i32,
}

impl Dog {
    fn new(name: String, age: i32) -> Dog {
        Dog { title: name, old: age }
    }
}

impl Animal for Dog {
    fn get_name(&self) -> String {
        self.title.clone()
    }

    fn get_age(&self) -> i32 {
        self.old
    }
}

impl Pet for Dog {
    fn show_affection(&self) {
        println!("This Dog instance affected by Pet.");
    }
}

fn print_animal_name_gen<T>(a: &T)
where
    T: Animal + ?Sized,
{
    println!("{}", a.get_name());
}

fn print_animal_name_dyn(a: &dyn Animal) {
    println!("{}", a.get_name());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let denis = Human::new("Denis".to_owned(), 16);
    let chip = Dog::new("Chip".to_owned(), 4);

    let animal: &dyn Animal;

    if args.len() > 1 && args[1] == "human" {
        animal = &denis;
    } else {
        animal = &chip;
    }

    print_animal_name_gen(animal);
    print_animal_name_dyn(animal);
    chip.show_affection();

    let undefined_animal = Pet::new("Can be treated as Pet".to_owned(), age: 3);
}
*/


/*
use std::env;

trait Animal {
    fn get_name(&self) -> String {
        "no name".to_owned()
    }
    fn get_age(&self) -> i32;
}

#[derive(Clone)]
struct Human {
    pub name: String,
    pub age: i32,
}

impl Human {
    fn new(name: String, age: i32) -> Human {
        Human { name, age }
    }
}

impl Animal for Human {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_age(&self) -> i32 {
        self.age
    }
}

trait Pet: Animal {
    fn show_affection(&self) {
        println!("I affect every Pet instance.");
    }
}

#[derive(Clone)]
struct Dog {
    title: String,
    old: i32,
}

impl Dog {
    fn new(name: String, age: i32) -> Dog {
        Dog { title: name, old: age }
    }
}

impl Animal for Dog {
    fn get_name(&self) -> String {
        self.title.clone()
    }

    fn get_age(&self) -> i32 {
        self.old
    }
}

impl Pet for Dog {

    // override the show_affection method of trait Pet
    fn show_affection(&self) {
        println!("This Dog instance affected by Pet.");
    }
}

#[derive(Clone)]
struct UndefinedPet {
    name: String,
    age: i32,
}

impl UndefinedPet {
    fn new(name: String, age: i32) -> UndefinedPet {
        UndefinedPet { name, age }
    }
}

impl Animal for UndefinedPet {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_age(&self) -> i32 {
        self.age
    }
}

impl Pet for UndefinedPet {

    // does not override the show_method()
    // fn show_affection(&self) {
    //     println!("This undefined Pet instance affected by Pet.");
    // }
}

fn print_animal_name_gen<T>(a: &T)
where
    T: Animal + ?Sized,
{
    println!("{}", a.get_name());
}

fn print_animal_name_dyn(a: &dyn Animal) {
    println!("{}", a.get_name());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let denis = Human::new("Denis".to_owned(), 16);
    let chip = Dog::new("Chip".to_owned(), 4);
    let undefined_pet = UndefinedPet::new("Can be treated as Pet".to_owned(), 3);

    let animal: &dyn Animal;

    if args.len() > 1 && args[1] == "human" {
        animal = &denis;
    } else {
        animal = &chip;
    }

    print_animal_name_gen(animal);
    print_animal_name_dyn(animal);
    chip.show_affection();

    print_animal_name_gen(&undefined_pet);
    print_animal_name_dyn(&undefined_pet);
    undefined_pet.show_affection();
}
*/



/*
// attempt to make progress bar (percentage for file downloading of tokio rqwest)
use std::io;
use tokio::{fs::File, io::AsyncWriteExt};
use reqwest::Client;
use dirs;

async fn fetch_image_data(url: &str) -> io::Result<Vec<u8>> {
    let client = Client::new();
    let response = client.get(url).send().await.unwrap();
    let mut image_data = Vec::new();

    let mut stream = response.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        image_data.extend_from_slice(&chunk);
    }

    Ok(image_data)
}

async fn save_image_data_to_downloads(image_data: Vec<u8>) -> io::Result<()> {
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
*/




/*
// bytes_stream not found
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("starting");

    let mut stream = reqwest::get("http://localhost:8080/").await?.bytes_stream();

    let mut buffer = Vec::new();
    while let Some(item) = stream.next().await {
        for byte in item? {
            // Might need to consider carriage returns too depending
            // on how the server is expected to send the data.
            if byte == b'\n' {
                println!("Got chunk: {:?}", buffer.as_slice());
                buffer.clear();
            } else {
                buffer.push(byte);
            }
        }
    }

    Ok(())
}
*/



/*
// https://stackoverflow.com/questions/62462036/how-to-convert-a-bytes-iterator-into-a-stream-in-rust
use futures::prelude::*;
use std::{
    fs::File,
    io::{self, prelude::*, BufReader},
};
use tokio;

fn async_read() -> impl Stream<Item = Result<u8, std::io::Error>> {
    let downloads_dir = dirs::download_dir().ok_or(io::Error::new(
        io::ErrorKind::NotFound,
        "Download directory not found",
    )).unwrap();
    let file_path = downloads_dir.join("blink.uf2"); // Provide the name of the file you want to read
    let f = File::open(file_path).expect("Could not open file");
    let reader = BufReader::new(f);
    stream::iter(reader.bytes())
}

async fn async_main() {
    while let Some(b) = async_read().next().await {
        println!("{:?}", b);
    }
}

#[tokio::main]
async fn main() {
    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    async_main().await;
}

*/



/*
// STATUS: err: captured variable cannot escape `FnMut` closure body
use futures::prelude::*;
use std::{
    fs::File,
    io::{self, prelude::*, BufReader},
    path::PathBuf,
};
use tokio;
use tokio::time::Duration;

async fn async_read(file_path: PathBuf) -> io::Result<impl Stream<Item = Result<u8, std::io::Error>>> {
    let f = File::open(&file_path)?;
    let reader = BufReader::new(f);
    let stream = stream::iter(reader.bytes())
        .map(|byte_result| {
            tokio::time::sleep(Duration::from_millis(100)).await; // Simulate some processing time
            byte_result
        });
    Ok(stream)
}

async fn async_main(file_path: PathBuf) -> io::Result<()> {
    let stream = async_read(file_path).await?;
    
    let mut progress = 0;
    stream
        .for_each(|byte_result| async {
            match byte_result {
                Ok(byte) => {
                    progress += 1;
                    // Update progress bar or any other progress indicator here
                    println!("Progress: {} bytes read", progress);
                }
                Err(err) => {
                    eprintln!("Error reading file: {}", err);
                }
            }
        })
        .await;

    Ok(())
}

#[tokio::main]
async fn main() {
    let file_path = dirs::download_dir().unwrap().join("your_file_name_here"); // Provide the name of the file you want to read
    match async_main(file_path).await {
        Ok(_) => println!("File fetch completed."),
        Err(err) => eprintln!("Error fetching file: {}", err),
    }
}
*/



/*
// fixed:
use futures::prelude::*;
use std::{
    fs::File,
    io::{self, prelude::*, BufReader},
    path::PathBuf,
};
use tokio;
use tokio::time::Duration;

async fn async_read(file_path: PathBuf) -> io::Result<impl Stream<Item = Result<u8, std::io::Error>>> {
    let f = File::open(&file_path)?;
    let reader = BufReader::new(f);
    let stream = stream::iter(reader.bytes())
        .map(|byte_result| {
            byte_result.map(|byte| {
                // tokio::time::sleep(Duration::from_millis(100)).await; // Simulate some processing time
                byte
            })
        });
    Ok(stream)
}

async fn async_main(file_path: PathBuf) -> io::Result<()> {
    let stream = async_read(file_path).await?;
    
    let progress = stream
        .fold(0, |progress, byte_result| async move {
            match byte_result {
                Ok(_) => {
                    let new_progress = progress + 1;
                    // Update progress bar or any other progress indicator here
                    println!("Progress: {} bytes read", new_progress);
                    new_progress
                }
                Err(err) => {
                    eprintln!("Error reading file: {}", err);
                    progress
                }
            }
        })
        .await;

    Ok(())
}

#[tokio::main]
async fn main() {
    let main_time: tokio::time::Instant = tokio::time::Instant::now();

    let file_path = dirs::document_dir().unwrap().join("example.txt"); // Provide the name of the file you want to read
    match async_main(file_path).await {
        Ok(_) => println!("File fetch completed."),
        Err(err) => eprintln!("Error fetching file: {}", err),
    }

    // End of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\n⌛️ Took: {:?} ({:?} ms)", duration, elapsed_ms);
}
*/



/*
// working fine for internet file
use futures::prelude::*;
use reqwest;
use std::io;
use tokio;

// async fn async_read(url: &str) -> io::Result<impl Stream<Item = Result<u8, std::io::Error>>> {
//     let response = reqwest::get(url).await.unwrap();
//     let body = response.bytes().await.unwrap();
//     let stream = stream::iter(body.into_iter().map(|byte| Ok(byte)));
//     Ok(stream)
// }
async fn async_read(url: &str) -> io::Result<impl Stream<Item = Result<u8, std::io::Error>>> {
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

    let body = response.bytes().await.unwrap();
    let stream = stream::iter(body.into_iter().map(|byte| Ok(byte)));
    Ok(stream)
}

async fn async_main(url: &str) -> io::Result<()> {
    let stream = async_read(url).await?;
    
    let progress = stream
        .fold(0, |progress, byte_result| async move {
            match byte_result {
                Ok(_) => {
                    let new_progress = progress + 1;
                    // Update progress bar or any other progress indicator here
                    // println!("Progress: {} bytes read", new_progress);
                    new_progress
                }
                Err(err) => {
                    eprintln!("Error reading file: {}", err);
                    progress
                }
            }
        })
        .await;

    Ok(())
}

#[tokio::main]
async fn main() {
    let url = "https://images.dog.ceo/breeds/mountain-swiss/n02107574_1387.jpg"; // Replace with the actual URL of the file
    match async_main(url).await {
        Ok(_) => println!("File fetch completed."),
        Err(err) => eprintln!("Error fetching file: {}", err),
    }
}
*/




/*
// 1 err, trying to fix but not succeeded yet (error[E0599]: no method named `next` found for opaque type `impl futures_util::Future<Output = Result<bytes::bytes::Bytes, reqwest::Error>>` in the current scope)
// use futures::prelude::*;
// use futures_util::FutureExt;    // not this
// use futures::StreamExt; // not this
use futures_util::TryStreamExt;
use reqwest;
use std::fs::File;
use std::io::{self, prelude::*};
use std::path::PathBuf;
use std::time::Duration;
use tokio;
use indicatif::{ProgressBar, ProgressStyle};

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
    let mut downloaded = 0;

    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .progress_chars("#>-"),
    );

    let mut writer = File::create(&file_path)?;

    let mut stream = response.bytes();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.unwrap();
        writer.write_all(&chunk)?;

        downloaded += chunk.len() as u64;
        pb.set_position(downloaded);

        // Simulate some processing time
        tokio::time::sleep(Duration::from_millis(12)).await;
    }

    pb.finish_with_message("Downloaded");
    Ok(())
}

#[tokio::main]
async fn main() {
    let url = "https://images.dog.ceo/breeds/mountain-swiss/n02107574_1387.jpg"; // Replace with the actual URL of the file
    let file_path = dirs::download_dir().unwrap().join("your_file_name_here"); // Replace with the name you want to save the file as
    match async_read_with_progress(url, file_path).await {
        Ok(_) => println!("File downloaded successfully."),
        Err(err) => eprintln!("Error downloading file: {}", err),
    }
}
*/




/*
// STATUS: not showing progress bar until finish, then it show 100% progress.
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
    // 25Mb pdf file: http://research.nhm.org/pdfs/10840/10840.pdf
    let url = "http://research.nhm.org/pdfs/10840/10840.pdf"; // Replace with the actual URL of the file
    let file_path = dirs::download_dir().unwrap().join("manual.pdf"); // Replace with the name you want to save the file as
    match async_read_with_progress(url, file_path).await {
        Ok(_) => println!("File downloaded successfully."),
        Err(err) => eprintln!("Error downloading file: {}", err),
    }
}
*/





/*
// DARN IT, worked !!!! flawlessly !!!
// Cargo.toml: dirs = "5.0.1", indicatif = "0.17.8", tokio = { version = "1", features = ["full"] }, reqwest = { version = "0.11.25", features = ["stream"]}, wasm-streams = "0.4.0"
use futures::StreamExt;
// https://github.com/rwf2/Rocket/discussions/1902
use reqwest;
use std::fs::File;
use std::io::{self, prelude::*};
use std::path::PathBuf;
use indicatif::{ProgressBar, ProgressStyle};

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

    let pb = ProgressBar::new(total_size);
    // pb.set_style(ProgressStyle::default_bar()
    //     .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
    //     .progress_chars("#>-"));
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        // .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));

    let mut writer = File::create(&file_path)?;

    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let bytes = chunk.unwrap();     // <-- fix here
        // if use "chunk?"  -> error: unknow size at compilation time

        writer.write_all(&bytes)?;
        pb.inc(bytes.len() as u64);
    }

    pb.finish_with_message("Downloaded");
    Ok(())
}

#[tokio::main]
async fn main() {
    // 1.Mb pdf avr user manual: http://research.nhm.org/pdfs/10840/10840.pdf
    let url = "http://research.nhm.org/pdfs/10840/10840.pdf"; // Replace with the actual URL of the file
    let file_path = dirs::download_dir().unwrap().join("manual.pdf"); // Replace with the name you want to save the file as
    match async_read_with_progress(url, file_path).await {
        Ok(_) => println!("File downloaded successfully."),
        Err(err) => eprintln!("Error downloading file: {}", err),
    }
}
*/





/*
// DAMN IT: to use bytes_stream(), we have to enable feature "stream", and that requires "wasm-streams"
// wasm-streams = "0.4.0"
// reqwest = { version = "0.11.25", features = ["stream"]}
use std::error::Error;

use reqwest::Response;
use tokio::io::{self, AsyncBufReadExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    // let client = reqwest::Client::new();

    // let response = client.get("STREAMING_URL")
    //     .send()
    //     .await.unwrap()
    //     .bytes_stream();

    //     if response.status().is_success() {
    //         println!("Successfully connected to host.");
    //     } else {
    //         eprintln!("Failed to connect to host: {}", response.status());
    //         return Err(io::Error::new(
    //             io::ErrorKind::Other,
    //             "Failed to connect to host",
    //         ));
    //     }
    
    //     let total_size = response.content_length().unwrap_or(0);
    //     println!("file size: {}", total_size);

    let mut res: Response = reqwest::get("https://online.hcmue.edu.vn")
        .await
        .unwrap();


    // match res.content_length() {
    //     Some(bytes) => println!("file size: {}", bytes),
    //     None => println!("The server didn’t send a content-length header.")
    // }

    println!("Status: {}", &res.status());
    println!("HTTP version: {:?}", &res.version());
    println!("Headers: {:?}", &res.headers());
    println!("RemoteAddr: {:?}", &res.remote_addr().unwrap());

    // while let Some(chunk) = res.chunk().await? {
    //     println!("Chunk: {chunk:?}");
    // }



    // while let Some(line) = lines.next_line().await? {
    //     println!("length = {}", line.len())
    // }

    Ok(())
}
*/





/*
// https://users.rust-lang.org/t/rust-doesnt-have-a-size-known-at-compile-time/48375
use std::fmt::Debug;

// definitely won't compile:
// #[derive(Debug)]
// struct Node<K: Debug,V: Debug> {
//     key: K,
//     value: V,
// }

// fn myprint<K:Debug + ?Sized, V: Debug + ?Sized>(node: &Node<K, V>) {
//     println!("{:?}", node);
// }

// attempt (worked):
// #[derive(Debug)]
// struct Node<'a, K: Debug + ?Sized,V: Debug + ?Sized> {
//     key: &'a K,     // added this lifetime 'a to resolve: " borrowed types always have a statically known size: `&` "
//     value: V,
// }

// fn myprint<K: Debug + ?Sized, V: Debug + ?Sized>(node: &Node<K, V>) {
//     println!("{:?}", node);
// }

// a working solution
#[derive(Debug)]
struct Node<'a, K: Debug + ?Sized, V: Debug + ?Sized> {
    key: &'a K,
    value: &'a V,
}

fn myprint<K: Debug + ?Sized, V: Debug + ?Sized>(node: &Node<K, V>) {
    println!("{:?}", node);
}

fn main() {
    let node = Node{key: "xxx", value: "yyy"};
    myprint(&node);
}
*/




/*
// Rust program for converting decimal to hexadecimal
fn main() {
    let decimal_number = 10;
    let hexadecimal_string = format!("{:X}", decimal_number);
    println!("Decimal {} is equivalent to hexadecimal {}", decimal_number, hexadecimal_string);
}
*/





/*
// indicatif example download:
use std::thread;
use std::time::Duration;
use std::{cmp::min, fmt::Write};

use indicatif::{ProgressBar, ProgressState, ProgressStyle};

fn main() {
    let mut downloaded = 0;
    let total_size = 70231231;

    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));

    while downloaded < total_size {
        let new = min(downloaded + 223211, total_size);
        downloaded = new;
        pb.set_position(new);
        thread::sleep(Duration::from_millis(12));
    }

    pb.finish_with_message("downloaded");
}
*/



/*
// colorfull bar
use std::thread;
use std::time::Duration;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rand::{thread_rng, Rng};

fn main() {
    let styles = [
        ("Rough bar:", "█  ", "red"),
        ("Fine bar: ", "█▉▊▋▌▍▎▏  ", "yellow"),
        ("Vertical: ", "█▇▆▅▄▃▂▁  ", "green"),
        ("Fade in:  ", "█▓▒░  ", "blue"),
        ("Blocky:   ", "█▛▌▖  ", "magenta"),
    ];

    let m = MultiProgress::new();

    let handles: Vec<_> = styles
        .iter()
        .map(|s| {
            let pb = m.add(ProgressBar::new(512));
            pb.set_style(
                ProgressStyle::with_template(&format!("{{prefix:.bold}}▕{{bar:.{}}}▏{{msg}}", s.2))
                    .unwrap()
                    .progress_chars(s.1),
            );
            pb.set_prefix(s.0);
            let wait = Duration::from_millis(thread_rng().gen_range(10..30));
            thread::spawn(move || {
                for i in 0..512 {
                    thread::sleep(wait);
                    pb.inc(1);
                    pb.set_message(format!("{:3}%", 100 * i / 512));
                }
                pb.finish_with_message("100%");
            })
        })
        .collect();

    for h in handles {
        let _ = h.join();
    }
}
*/




/*
// similar to npm compiles, advanced arguments: $ cr cargo build --release
use std::io::{BufRead, BufReader};
use std::process;
use std::time::{Duration, Instant};

use indicatif::{HumanDuration, ProgressBar, ProgressStyle};

pub fn main() {
    let started = Instant::now();

    println!("Compiling package in release mode...");

    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(200));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.dim.bold} cargo: {wide_msg}")
            .unwrap()
            .tick_chars("/|\\- "),
    );

    let mut p = process::Command::new("cargo")
        .arg("build")
        .arg("--release")
        .stderr(process::Stdio::piped())
        .spawn()
        .unwrap();

    for line in BufReader::new(p.stderr.take().unwrap()).lines() {
        let line = line.unwrap();
        let stripped_line = line.trim();
        if !stripped_line.is_empty() {
            pb.set_message(stripped_line.to_owned());
        }
        pb.tick();
    }

    p.wait().unwrap();

    pb.finish_and_clear();

    println!("Done in {}", HumanDuration(started.elapsed()));
}
*/




/*
// single colored bar
use std::{cmp::min, thread};
use std::time::Duration;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rand::{thread_rng, Rng};

fn main() {

    let mut downloaded = 0;
    let total_size = 70231231;

    let s = ("Vertical: ", "█▇▆▅▄▃▂▁  ", "yellow");

    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::with_template(&format!("{{prefix:.bold}}▕{{bar:.{}}}▏{{msg}}", s.2))
            .unwrap()
            .progress_chars(s.1),
    );

    while downloaded < total_size {
        let new = min(downloaded + 223211, total_size);
        downloaded = new;
        pb.set_position(new);
        thread::sleep(Duration::from_millis(12));
        
    }
    

    pb.finish_with_message("downloaded");
}
*/




/*
// err panic
use std::arch::asm;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let main_time: std::time::Instant = std::time::Instant::now();

    let i: u64 = 3;
    let o: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));

    let o_clone = o.clone();
    tokio::spawn(async move {
        unsafe {
            asm!(
                "mov {0}, {1}",
                "add {0}, 5",
                out(reg) *o_clone.lock().await,
                in(reg) i,
            );
        }
    });

    // Multiply x by 6 using shifts and adds
    let mut x: u64 = 4;
    let x_mutex = Arc::new(Mutex::new(x));

    let x_clone = x_mutex.clone();
    tokio::spawn(async move {
        unsafe {
            asm!(
                "mov {tmp}, {x}",
                "shl {tmp}, 1",
                "shl {x}, 2",
                "add {x}, {tmp}",
                x = inout(reg) *x_clone.lock().await,
                tmp = out(reg) _,
            );
        }
    });

    // Wait for both tasks to complete  (ERR HERE)
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let o_result = o.lock().await;
        let x_result = x_mutex.lock().await;
        println!("Output for first block: {}", *o_result);
        println!("Output for second block: {}", *x_result);
    });

    // End of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\n⌛️ Execution time: {:?} ({:?} ms)", duration, elapsed_ms);
}
*/




/*
// STATUS: worked but not very performant
// inline-asm rust with Tokio async concurrent parallel
use std::{arch::asm, sync::Arc};
use tokio::sync::{mpsc, Mutex};

async fn process_asm_block(i: u64) -> u64 {
    let o: u64;
    unsafe {
        asm!(
            "mov {0}, {1}",
            "add {0}, 5",
            out(reg) o,
            in(reg) i,
        );
    }
    o
}

async fn process_shift_block(mut x: u64) -> u64 {
    unsafe {
        asm!(
            "mov {tmp}, {x}",
            "shl {tmp}, 1",
            "shl {x}, 2",
            "add {x}, {tmp}",
            x = inout(reg) x,
            tmp = out(reg) _,
        );
    }
    x
}

#[tokio::main]
async fn main() {
    let main_time: std::time::Instant = std::time::Instant::now();

    let i: u64 = 3;

    // Create channels for communication between tasks
    let (o_sender, mut o_receiver) = mpsc::channel(1);
    let (x_sender, mut x_receiver) = mpsc::channel(1);

    // Spawn asynchronous tasks
    let i_clone = i;
    tokio::spawn(async move {
        let result = process_asm_block(i_clone).await;
        o_sender.send(result).await.expect("Failed to send result");
    });

    let x: u64 = 4;
    tokio::spawn(async move {
        let result = process_shift_block(x).await;
        x_sender.send(result).await.expect("Failed to send result");
    });

    // Receive and print results
    let o = o_receiver.recv().await.expect("Failed to receive result");
    let x = x_receiver.recv().await.expect("Failed to receive result");

    println!("Output for first block: {}", o);
    println!("Output for second block: {}", x);

    // End of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\n⌛️ Execution time: {:?} ({:?} ms)", duration, elapsed_ms);
}
*/




/*
// rust inline assembly with Tokio async concurrent parallel
// use tokio::join! to optimize performance
use std::arch::asm;
use tokio::sync::mpsc;

async fn first_block(i: u64) -> u64 {
    let mut o: u64 = 0;
    unsafe {
        asm!(
            "mov {0}, {1}",
            "add {0}, 5",
            out(reg) o,
            in(reg) i,
        );
    }
    o
}

async fn second_block(mut x: u64) -> u64 {
    unsafe {
        asm!(
            "mov {tmp}, {x}",
            "shl {tmp}, 1",
            "shl {x}, 2",
            "add {x}, {tmp}",
            x = inout(reg) x,
            tmp = out(reg) _,
        );
    }
    x
}

#[tokio::main]
async fn main() {
    let main_time = std::time::Instant::now();

    let i: u64 = 3;
    let x: u64 = 4;

    // Create channels for communication
    let (tx1, rx1) = mpsc::channel::<u64>(1);
    let (tx2, rx2) = mpsc::channel::<u64>(1);

    // Spawn tasks
    let first_task = tokio::spawn(async move {
        first_block(i).await
    });

    let second_task = tokio::spawn(async move {
        second_block(x).await
    });

    // Wait for tasks to complete and collect results
    let (result1, result2) = tokio::try_join!(first_task, second_task).unwrap();

    println!("Output for first block: {}", result1);
    println!("Output for second block: {}", result2);

    // End of main
    let duration = main_time.elapsed();
    let elapsed_ms = duration.as_secs_f64() * 1000.0;
    println!("\n⌛️ Execution time: {:?} ({:.6} ms)", duration, elapsed_ms);
}
*/
/*
Output:
Output for first block: 8
Output for second block: 24

⌛️ Execution time: 135.858µs (0.135858 ms)
*/






/*
// a review of async deep dive (tokio):
// https://blog.yanick.site/2020/09/08/rust/poller-in-tokio/
use tokio::{io::{AsyncRead, AsyncReadExt, AsyncWriteExt, ReadBuf}, net::TcpListener};
use std::{env, error::Error, io::{self, Read}, pin::Pin, task::{Context, Poll}, thread};
use std::future::Future;


// impl<R> Future for Read<'_, R>
// where
//     R: AsyncRead + Unpin + ?Sized,
// {
//     type Output = io::Result<usize>;

//     fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<usize>> {
//         let me = &mut *self;
//         let mut buf = ReadBuf::new(me.buf);
//         // ready!(Pin::new(&mut *me.reader).poll_read(cx, &mut buf))?; 方便理解我把宏展开了
//         match Pin::new(&mut *me.reader).poll_read(cx, &mut buf) {
//             std::task::Poll::Ready(t) => t,
//             std::task::Poll::Pending => return std::task::Poll::Pending,
//         }
//         Poll::Ready(Ok(buf.filled().len()))
//     }
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let mut listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Listening on: {}", addr);

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("accepted {}", addr);

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = socket
                    .read(&mut buf) // import io::AsyncReadEx
                    .await
                    .expect("failed to read data from socket");

                println!("current thread: {:?}", thread::current());

                if n == 0 {
                    return;
                }

                socket
                    .write_all(&buf[0..n])  // import io::AsyncWriteExt
                    .await
                    .expect("failed to write data to socket");
            }
        });
    }
}
*/






/*
// https://stackoverflow.com/questions/31497584/how-can-i-wrap-another-type-and-add-fields-and-methods-to-it
use std::ops::{Deref, DerefMut};

struct Glyph;

struct Glyphs(Vec<Glyph>);

impl Glyphs {
    fn new() -> Self {
        Glyphs(vec![])
    }
}

impl Deref for Glyphs {
    type Target = Vec<Glyph>;
    fn deref(&self) -> &Vec<Glyph> { &self.0 }
}

impl DerefMut for Glyphs {
    fn deref_mut(&mut self) -> &mut Vec<Glyph> { &mut self.0 }
}

fn main() {
    let mut gs = Glyphs::new();
    gs.push(Glyph);
    gs.push(Glyph);
    println!("gs.len: {}", gs.len());
}
*/



/*
// https://stackoverflow.com/questions/33376486/is-there-a-way-other-than-traits-to-add-methods-to-a-type-i-dont-own?rq=3
trait DoubleExt {
    fn double(&self) -> Self;
}

impl DoubleExt for i32 {
    fn double(&self) -> Self {
        *self * 2
    }
}

impl DoubleExt for f64 {
    fn double(&self) -> Self {
        *self * 2.0
    }
}

fn main() {
    let a = 42;
    let b = 42.2;

    println!("{}", a.double());
    println!("{}", b.double())
}
*/




/*
// Rust magazine
// Rust just copies instead of Move?
struct Massive {
    a: [i128; 10000],
}

impl Massive {
    #[inline(always)]
    fn stupid(mut self) -> Self {
        println!("{:?}", &mut self.a[1] as *mut i128); // 0x7ffe178babc0

        //do some stuff to alter it
        self.a[1] += 23;
        self.a[4] += 24;
        
        self
    }
}

fn main() {
    let mut f = Massive { a: [10i128; 10000] }; // 0x7ffe17845870

    println!("{:?}", &mut f.a[1] as *mut i128);

    let mut f2 = f.stupid();

    println!("{:?}", &mut f2.a[1] as *mut i128); // 0x7ffe17893ac0
}

// 作者：Rust_Magazine
// 链接：https://juejin.cn/post/7062249584067608612
// 来源：稀土掘金
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。
*/



/*
unsigned long myTime;
void setup() {
    Serial.begin(9600);
    Serial.print("Hello");
    Serial.print("Hello again");
}

void loop() {
    Serial.print("Time: ");
    myTime = millis();
    Serial.print(myTime);
    delay(1000);    // one sec

}
// Bấm verify (v) -> bấm nạp code (->)
// verify = compile
// góc phải trên IDE có mở serial để đọc output log
*/


/*
// blink onboard led:
// the setup function runs once when you press reset or power the board
void setup() {
    // initialize digital pin LED_BUILTIN as an output.
    pinMode(LED_BUILTIN, OUTPUT);
  }
  
  // the loop function runs over and over again forever
  void loop() {
    digitalWrite(LED_BUILTIN, HIGH);  // turn the LED on (HIGH is the voltage level)
    delay(300);                      // wait for 0.3 second
    digitalWrite(LED_BUILTIN, LOW);   // turn the LED off by making the voltage LOW
    delay(300);                      // wait for 0.3 second
  }
*/


// read button:
/*
  Button

  Turns on and off a light emitting diode(LED) connected to digital pin 13,
  when pressing a pushbutton attached to pin 2.

  The circuit:
  - LED attached from pin 13 to ground through 220 ohm resistor
  - pushbutton attached to pin 2 from +5V
  - 10K resistor attached to pin 2 from ground

  - Note: on most Arduinos there is already an LED on the board
    attached to pin 13.

  created 2005
  by DojoDave <http://www.0j0.org>
  modified 30 Aug 2011
  by Tom Igoe

  This example code is in the public domain.

  https://www.arduino.cc/en/Tutorial/BuiltInExamples/Button
*/


/*
// https://docs.arduino.cc/built-in-examples/digital/Button/
// constants won't change. They're used here to set pin numbers:
const int buttonPin = 2;  // the number of the pushbutton pin
const int ledPin = 13;    // the number of the LED pin (built-in)

// variables will change:
int buttonState = 0;  // variable for reading the pushbutton status

void setup() {
  // initialize the LED pin as an output:
  pinMode(ledPin, OUTPUT);
  // initialize the pushbutton pin as an input:
  pinMode(buttonPin, INPUT);
}

void loop() {
  // read the state of the pushbutton value:
  buttonState = digitalRead(buttonPin);

  // check if the pushbutton is pressed. If it is, the buttonState is HIGH:
  if (buttonState == HIGH) {
    // turn LED on:
    digitalWrite(ledPin, HIGH);
    delay(600);
  } else {
    // turn LED off:
    digitalWrite(ledPin, LOW);
    delay(600)
  }
}
*/





/*
// fast blink & sleep & repeat
// Define the pin connected to the LED
const int ledPin = 13;

void setup() {
  // Set the LED pin as an output
  pinMode(ledPin, OUTPUT);
}

void loop() {
  // Blink fast three times
  for (int i = 0; i < 3; i++) {
    digitalWrite(ledPin, HIGH); // Turn the LED on
    delay(100);                 // Wait for 200 milliseconds
    digitalWrite(ledPin, LOW);  // Turn the LED off
    delay(100);                 // Wait for 200 milliseconds
  }

  // Pause before repeating the pattern
  delay(600);  // Wait for 1 second
}
*/


/*
// servo 180 deg turn 
#include <Servo.h>

Servo myservo;  // create servo object to control a servo
// twelve servo objects can be created on most boards

int pos = 0;    // variable to store the servo position

void setup() {
  myservo.attach(9);  // attaches the servo on pin 9 to the servo object
}

void loop() {
  // Turn fast clockwise (90 degrees)
  myservo.write(180);
  delay(5);  // Wait for 5 milliseconds

  // Turn fast counterclockwise (90 degrees)
  myservo.write(0);
  delay(5);  // Wait for 5 milliseconds
}
*/



/*
//Arduino Code - Move from A to B by a specific number of steps with set speed and acceleration
// ref: https://www.diyengineers.com/2021/11/25/28byj-48-uln2003-stepper-motor-control/
 
// Include the Arduino Stepper.h library:
#include <AccelStepper.h> //Include the AccelStepper library
 
// Define the motor pins:
#define MP1  8 // IN1 on the ULN2003
#define MP2  9 // IN2 on the ULN2003
#define MP3  10 // IN3 on the ULN2003
#define MP4  11 // IN4 on the ULN2003
 
#define MotorInterfaceType 8 // Define the interface type as 8 = 4 wires * step factor (2 for half step)
AccelStepper stepper = AccelStepper(MotorInterfaceType, MP1, MP3, MP2, MP4);//Define the pin sequence (IN1-IN3-IN2-IN4)
const int SPR = 2048;//Steps per revolution
 
void setup() {
  stepper.setMaxSpeed(1200);//Set the maximum motor speed in steps per second
  stepper.setAcceleration(200);//Set the maximum acceleration in steps per second^2
}
 
void loop() {
  stepper.moveTo(4*SPR); //Set the target motor position (i.e. turn motor for 4 full revolutions)
  stepper.runToPosition(); // Run the motor to the target position 
  delay(1000);
  stepper.moveTo(-4*SPR);//Same as above: Set the target motor position (i.e. turn motor for 4 full revolutions)
  stepper.runToPosition(); // Run the motor to the target position 
  delay(10000);
}
*/





/*
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
            // print!("I’m being looped infinitely"); // breakline in terminal
        }
    });

    // Await the completion of the spawned Tokio task.
    reading_task.await.unwrap();

    // Ok(())
}
*/






/*
// STATUS: worked, but the the breakline is being executed before the stdout is fully flushed.
// explanatory: In an asynchronous Rust program using Tokio, we typically want the program to terminate when all tasks are completed. To achieve this, we can use a synchronization mechanism like channels to signal when the file reading task is finished. Here's how we can modify our code to use channels
use tokio::{fs::File, io::{self, AsyncReadExt, AsyncWriteExt}, sync::mpsc};

const LEN: usize = 16 * 1024; // 16 Kb

#[tokio::main]
async fn main() {
    let main_time = std::time::Instant::now();
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
            // // Add a breakline after printing the content.
            // println!();
        }

        // Send a signal through the channel to indicate that the task is finished.
        let _ = tx.send(()).await;
    });

    // Wait for the file reading task to finish.
    reading_task.await.unwrap();

    // Wait for the signal from the channel indicating that the task is finished.
    rx.recv().await;

    // Add a breakline after printing the content.
    println!();

    // End of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\n⌛️ Execution time: {:?} ({:?} ms)", duration, elapsed_ms);

    // The program will automatically terminate here.
}
*/





/*
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
*/





/*
// STATUS: This code sometimes cause deadlock
// by this approach of multiple `tokio::spawn()`, we create 4 concurrent async tasks that run toghether
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (sender1, mut receiver) = mpsc::channel::<String>(10); // Specify the type as String
    let sender2: mpsc::Sender<String> = sender1.clone();
    let sender3 = sender1.clone();
    let sender4 = sender1.clone();

    println!("max channel capacity: {}", &sender1.max_capacity());  // no definition in struct `UnboundedSender`

    // timer to measure 4 spawned concurrent tasks
    let time: std::time::Instant = std::time::Instant::now();

    let handle1 = tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        for i in 0..=10 {
            if let Err(_) = sender1.send(i.to_string()).await {
                println!("receiver dropped");
                return;
            }
            // task waits until the receiver receives a value.
        }
    });

    let handle2 = tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        for i in 11..=20 {
            if let Err(_) = sender2.send(i.to_string()).await {
                println!("receiver dropped");
                return;
            }
        }
    }).await.unwrap();


    let handle3 = tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        for i in 21..=30 {
            if let Err(_) = sender3.send(i.to_string()).await {
                println!("receiver dropped");
                return;
            }
        }
    });

    // tokio::join!(handle1, handle2, handle3);


    tokio::spawn(async move {
        // This will return an error and send
        // no message if the buffer is full
        let _ = sender4.try_send("from sender4".to_string());
    });

    while let Some(i) = receiver.recv().await {
        println!("got message = {}", i);
    }

    // Stop timer & print to terminal
    let duration: std::time::Duration = time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    let elapsed_seconds: f64 = elapsed_ms / 1000.0;
    println!(
        ">>>  time elapsed: {:?} ({:?} ms) ({:.8} s)",
        duration, elapsed_ms, elapsed_seconds
    );
}
*/





/*
// Protothread и кооперативная многозад c++
// FIX deadlock of mpsc_worker.rs:
// Explanatory: To prevent deadlock, we need to ensure that the receiver loop terminates even if there are no more messages in the channel. We can achieve this by introducing a sentinel value and having the senders send this value to signal the end of the stream.
use tokio::sync::mpsc;

const SENTINEL: &str = "__SENTINEL__";

#[tokio::main]
async fn main() {
    let (sender1, mut receiver) = mpsc::channel::<String>(10);
    let sender2 = sender1.clone();
    let sender3 = sender1.clone();
    let sender4 = sender1.clone();

    println!("max channel capacity: {}", sender1.capacity());

    let time = std::time::Instant::now();

    let handle1 = tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        for i in 0..=10 {
            if let Err(_) = sender1.send(i.to_string()).await {
                println!("receiver dropped");
                return;
            }
        }
        // Signal end of stream
        sender1.send(SENTINEL.to_string()).await.unwrap();
    });

    let handle2 = tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        for i in 11..=20 {
            if let Err(_) = sender2.send(i.to_string()).await {
                println!("receiver dropped");
                return;
            }
        }
        // Signal end of stream
        sender2.send(SENTINEL.to_string()).await.unwrap();
    });

    let handle3 = tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(30)).await;

        for i in 21..=30 {
            if let Err(_) = sender3.send(i.to_string()).await {
                println!("receiver dropped");
                return;
            }
        }
        // Signal end of stream
        sender3.send(SENTINEL.to_string()).await.unwrap();
    });

    tokio::spawn(async move {
        // This will return an error and send
        // no message if the buffer is full
        let _ = sender4.try_send("from sender4".to_string());
        // Signal end of stream
        sender4.send(SENTINEL.to_string()).await.unwrap();
    });

    let mut count = 0;
    while let Some(i) = receiver.recv().await {
        if i == SENTINEL {
            count += 1;
            if count == 4 {
                break;
            }
        } else {
            println!("got message = {}", i);
        }
    }

    let duration = time.elapsed();
    let elapsed_ms = duration.as_secs_f64() * 1000.0;
    let elapsed_seconds = elapsed_ms / 1000.0;
    println!(
        ">>>  time elapsed: {:?} ({:?} ms) ({:.8} s)",
        duration, elapsed_ms, elapsed_seconds
    );
}
*/




/*
// Try converting this to C/C++ code
// single threaded, blocking whole program
use std::sync::mpsc;
use std::thread;
use rand::prelude::*;


fn main() {
    let (tx, rx) = mpsc::channel::<String>();
	let mut rng = thread_rng();
	
	for i in 1..=3 {
		let tx = tx.clone();
	
		thread::spawn(move || {
      		let val = format!("hi from thread {}", i);
     		tx.send(val).unwrap();
    	});
		
        // Generate a random delay for each thread
        // let delay_ms = rng.gen_range(10..=100); // Random delay between 1000ms and 3000ms
        // std::thread::sleep(std::time::Duration::from_millis(delay_ms));
	}
	
	// without closing sender, it will deadlock
	drop(tx);
	
	
	// receive if there are messages.
	// damn it, the while loop made me struggled for a while
	while let Ok(msg) = rx.recv() {
		println!("Got {}", msg);
	}
}
*/
/*
Output:
Got hi from thread 1
Got hi from thread 2
Got hi from thread 3
*/



/*
// single threaded, blocking whole program
use std::sync::mpsc;
use std::thread;
use rand::prelude::*;


fn main() {
    let main_time = std::time::Instant::now();

    let (tx, rx) = mpsc::channel::<String>();
	let mut rng = thread_rng();
	
	for i in 1..=3 {
		let tx = tx.clone();
	
		thread::spawn(move || {
      		let val = format!("hi from thread {}", i);
     		tx.send(val).unwrap();
    	});
		
        // Generate a random delay for each thread
        let delay_ms = rng.gen_range(10..=200); // Random delay between 1000ms and 3000ms
        std::thread::sleep(std::time::Duration::from_millis(delay_ms));
	}

    for i in 4..=10 {
		let tx = tx.clone();
	
		thread::spawn(move || {
      		let val = format!("hi from thread {}", i);
     		tx.send(val).unwrap();
    	});
		
        // Generate a random delay for each thread
        // let delay_ms = rng.gen_range(10..=200); // Random delay between 1000ms and 3000ms
        // std::thread::sleep(std::time::Duration::from_millis(delay_ms));
	}
	
	// without closing sender, it will deadlock
	drop(tx);
	
	// only receives first 1 message.
	//let received = rx.recv().unwrap();
    //println!("Got: {}", received);
	
	
	// receive if there are messages.
	// damn it, the while loop made me struggled for a while
	while let Ok(msg) = rx.recv() {
		println!("Got {}", msg);
	}

    // End of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\n⌛️ Execution time: {:?} ({:?} ms)", duration, elapsed_ms);

}
*/




/*
// (BAD)achieved concurrent output but still blocking
// worked, but must low performant due to instance re-creation/cloning
use std::sync::mpsc;
use std::thread;
use rand::prelude::*;

fn main() {
    let (tx, rx) = mpsc::channel::<String>();
    // let mut rng = thread_rng();

    // Spawn threads to send messages
    for i in 1..=3 {
        let tx = tx.clone();

        thread::spawn(move || {
            let mut rng = thread_rng();

            let val = format!("hi from thread {}", i);
            tx.send(val).unwrap();

            // Generate a random delay for each thread
            let delay_ms = rng.gen_range(10..=200);
            std::thread::sleep(std::time::Duration::from_millis(delay_ms));
        });
    }

    for i in 4..=10 {
        let tx = tx.clone();

        thread::spawn(move || {
            let mut rng = thread_rng();

            let val = format!("hi from thread {}", i);
            tx.send(val).unwrap();

            // Generate a random delay for each thread
            let delay_ms = rng.gen_range(10..=200);
            std::thread::sleep(std::time::Duration::from_millis(delay_ms));
        });
    }

    // Spawn a separate thread to handle printing
    thread::spawn(move || {
        // Receive messages and print them as they arrive
        while let Ok(msg) = rx.recv() {
            println!("Got {}", msg);
        }
    });

    // (Bad) Sleep in the main thread to allow other threads to run
    std::thread::sleep(std::time::Duration::from_secs(1));
}
*/
/*
Output:
Got hi from thread 6
Got hi from thread 8
Got hi from thread 5
Got hi from thread 7
Got hi from thread 1
Got hi from thread 4
Got hi from thread 3
Got hi from thread 9
Got hi from thread 2
Got hi from thread 10
*/




/*
// error: `Rc<UnsafeCell<ReseedingRng<rand_chacha::chacha::ChaCha12Core, OsRng>>>` cannot be sent between threads safely
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use rand::{prelude::*, thread_rng};

fn main() {
    let (tx, rx) = mpsc::channel::<String>();
    let rng = Arc::new(Mutex::new(thread_rng()));

    // Spawn threads to send messages
    for i in 1..=3 {
        let tx = tx.clone();
        let rng = Arc::clone(&rng);

        thread::spawn(move || {
            let mut rng = rng.lock().unwrap();

            let val = format!("hi from thread {}", i);
            tx.send(val).unwrap();

            // Generate a random delay for each thread
            let delay_ms = rng.gen_range(10..=200);
            std::thread::sleep(std::time::Duration::from_millis(delay_ms));
        });
    }

    for i in 4..=10 {
        let tx = tx.clone();
        let rng = Arc::clone(&rng);

        thread::spawn(move || {
            let mut rng = rng.lock().unwrap();

            let val = format!("hi from thread {}", i);
            tx.send(val).unwrap();

            // Generate a random delay for each thread
            let delay_ms = rng.gen_range(10..=200);
            std::thread::sleep(std::time::Duration::from_millis(delay_ms));
        });
    }

    // Spawn a separate thread to handle printing
    thread::spawn(move || {
        // Receive messages and print them as they arrive
        while let Ok(msg) = rx.recv() {
            println!("Got {}", msg);
        }
    });

    // Sleep in the main thread to allow other threads to run
    std::thread::sleep(std::time::Duration::from_secs(1));
}
*/




/*
// worked, achived both concurrent & yielding / nonblocking
// Explanatory: the issue is that ThreadRng, which is the type returned by thread_rng(), doesn't implement Send, which means it can't be sent across threads. To work around this limitation, you can create a new random number generator for each thread. However, since you're using rand::thread_rng(), which returns ThreadRng, you'll need to use rand::SeedableRng to create new random number generators for each thread. Here's how you can refactor the code to achieve this:
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use rand::{prelude::*, rngs::StdRng, SeedableRng};

fn main() {
    let (tx, rx) = mpsc::channel::<String>();
    let seed = rand::thread_rng().gen();
    let rng = Arc::new(Mutex::new(StdRng::seed_from_u64(seed)));

    // Spawn threads to send messages
    for i in 1..=3 {
        let tx = tx.clone();
        let rng = Arc::clone(&rng);

        thread::spawn(move || {
            let mut rng = rng.lock().unwrap();

            let val = format!("hi from thread {}", i);
            tx.send(val).unwrap();

            // Generate a random delay for each thread
            let delay_ms = rng.gen_range(10..=200);
            std::thread::sleep(std::time::Duration::from_millis(delay_ms));
        });
    }

    for i in 4..=10 {
        let tx = tx.clone();
        let seed = rand::thread_rng().gen();
        let rng = Arc::new(Mutex::new(StdRng::seed_from_u64(seed)));

        thread::spawn(move || {
            let mut rng = rng.lock().unwrap();

            let val = format!("hi from thread {}", i);
            tx.send(val).unwrap();

            // Generate a random delay for each thread
            let delay_ms = rng.gen_range(10..=200);
            std::thread::sleep(std::time::Duration::from_millis(delay_ms));
        });
    }

    // Spawn a separate thread to handle printing
    thread::spawn(move || {
        // Receive messages and print them as they arrive
        while let Ok(msg) = rx.recv() {
            println!("Got {}", msg);
        }
    });

    // Sleep in the main thread to allow other threads to run
    std::thread::sleep(std::time::Duration::from_secs(1));
}
*/
/*
Output 1:
Got hi from thread 1
Got hi from thread 4
Got hi from thread 5
Got hi from thread 6
Got hi from thread 8
Got hi from thread 7
Got hi from thread 9
Got hi from thread 10
Got hi from thread 2
Got hi from thread 3
Output 2:
Got hi from thread 1
Got hi from thread 6
Got hi from thread 5
Got hi from thread 7
Got hi from thread 8
Got hi from thread 9
Got hi from thread 10
Got hi from thread 4
Got hi from thread 3
Got hi from thread 2
*/




/*
// we indeed now use total of 3 threads (sender1, sender2, receiver)
// produced good code now, tips: try make a tokio async version
// in an attempt to make equivalent example for C chan example:
use std::sync::{mpsc, Arc, Mutex};
use std::thread;


fn main() {
    let main_time: std::time::Instant = std::time::Instant::now();

    let (tx, rx) = mpsc::channel::<String>();
    let tx2 = tx.clone();

    let sender_thread1 = thread::spawn(move || {
        println!("This is sender_thread, {:?}", std::thread::current().id());
        let tx: mpsc::Sender<String> = tx.clone();
        
        for i in 1..=3 {
            let val: String = format!("hi from task {}, thread {:?}", i, std::thread::current().id());
            match tx.send(val) {
                Ok(_) => println!("Sent"),
                Ok(_) => {},
                Err(e) => {
                    // The channel is closed, so the receiver should exit
                    println!("Failed to send: {}", e);
                    // break;
                }
            };
        }
        println!("sender_thread ({:?}) exit now...", std::thread::current().id());
    });
    
    // this works !!! terminates
    let sender_thread2 = thread::spawn(move || {
        println!("This is sender_thread, {:?}", std::thread::current().id());
        let tx: mpsc::Sender<String> = tx2.clone();
        
        for i in 4..=10 {
            let val: String = format!("hi from task {}, thread {:?}", i, std::thread::current().id());
            match tx.send(val) {
                Ok(_) => println!("Sent"),
                Ok(_) => {},
                Err(e) => {
                    // The channel is closed, so the receiver should exit
                    println!("Failed to send: {}", e);
                    // break;
                }
            };
        }
        println!("sender_thread ({:?}) exit now...", std::thread::current().id());
    });


    // Spawn a separate thread to handle receiving & join the receiver thread
    let receiver_thread = thread::spawn(move || {
        println!("This is receiver_thread");

        loop {
            match rx.recv() {
                Ok(mes) => println!("Got {}", mes),
                Err(_) => {
                    // The channel is closed, so the receiver should exit
                    println!("Channel closed. Nothing to receive, receiver exiting....");
                    break;
                }
            };
        }
        
        println!("receiver_thread exit now...");
    });
    receiver_thread.join();
    // sender_thread.join();    // doesn't really needed to

    // works
    // while let Ok(msg) = rx.recv() {
    //     println!("Got {}", msg);
    // }

    // End of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\n⌛️ Execution time: {:?} ({:?} ms)", duration, elapsed_ms);
    
}
*/
/*
Output:
This is sender_thread, ThreadId(2)
This is receiver_thread
Sent
Sent
Sent
sender_thread (ThreadId(2)) exit now...
This is sender_thread, ThreadId(3)
Got hi from task 1, thread ThreadId(2)
Got hi from task 2, thread ThreadId(2)
Got hi from task 3, thread ThreadId(2)
Got hi from task 4, thread ThreadId(3)
Sent
Sent
Sent
Sent
Got hi from task 5, thread ThreadId(3)
Got hi from task 6, thread ThreadId(3)
Got hi from task 7, thread ThreadId(3)
Sent
Sent
Got hi from task 8, thread ThreadId(3)
Got hi from task 9, thread ThreadId(3)
Got hi from task 10, thread ThreadId(3)
Sent
sender_thread (ThreadId(3)) exit now...
Channel closed. Nothing to receive, receiver exiting....
receiver_thread exit now...

⌛️ Execution time: 490.095µs (0.49009499999999995 ms)
*/




/*
// STATUS: deadlock
use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::collections::VecDeque;
use std::time::Duration;
use std::time::Instant;

fn sender(id: i32, messages: Arc<(Mutex<VecDeque<String>>, Condvar)>) {
    let (mtx, consumer_cv) = &*messages;
    let mut messages_queue = mtx.lock().unwrap();
    
    for i in (id * 10 - 9)..=(id * 10) {
        let msg = format!("This is thread {}, message {}", id, i);
        messages_queue.push_back(msg);
        consumer_cv.notify_one();
    }
}

fn main() {
    // Start timer
    let start = Instant::now();

    let messages = Arc::new((Mutex::new(VecDeque::new()), Condvar::new()));
    let messages_clone = Arc::clone(&messages);

    let mut threads = vec![];

    for i in 1..=3 {
        let messages_clone = Arc::clone(&messages);
        threads.push(thread::spawn(move || {
            sender(i, messages_clone);
        }));
    }

    let receiver = thread::spawn(move || {
        let (mtx, consumer_cv) = &*messages_clone;
        let mut messages_queue = mtx.lock().unwrap();
        loop {
            let timeout = Duration::from_millis(100); // Adjust timeout as needed
            let result = consumer_cv.wait_timeout(messages_queue, timeout).unwrap();
            messages_queue = result.0;
            if !messages_queue.is_empty() {
                if let Some(msg) = messages_queue.pop_front() {
                    println!("Got {}", msg);
                }
            } else {
                break;
            }
        }
    });

    for thread in threads {
        thread.join().unwrap();
    }

    let (mtx, consumer_cv) = &*messages;
    let mut messages_queue = mtx.lock().unwrap();
    *messages_queue = VecDeque::new(); // Clear the queue
    consumer_cv.notify_one(); // Notify the receiver to exit the loop

    receiver.join().unwrap();

    // Stop timer
    let duration = start.elapsed();
    println!(
        "took {}µs ({}ms) ({}s) to execute",
        duration.as_micros(),
        duration.as_millis(),
        duration.as_secs_f64()
    );
}
*/



/* 
// try this new code
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc;
use std::time::Instant;

fn sender(id: i32, sender_channel: mpsc::Sender<String>) {
    let cur_thread_id = std::thread::current().id();
    for i in (id * 10 - 9)..=(id * 10) {
        let msg = format!("This is thread {:?}, message {}", cur_thread_id, i);
        sender_channel.send(msg).unwrap();
    }
}

fn main() {
    // Start timer
    let start = Instant::now();

    let (sender_tx, receiver_rx) = mpsc::channel::<String>();
    let receiver_rx = Arc::new(Mutex::new(receiver_rx));

    let mut threads = vec![];

    for i in 1..=3 {
        let sender_clone = sender_tx.clone();
        // let receiver_clone = Arc::clone(&receiver_rx);
        threads.push(thread::spawn(move || {
            sender(i, sender_clone);
        }));
    }

    let receiver_thread = thread::spawn(move || {
        loop {
            let msg = {
                let receiver_lock = receiver_rx.lock().unwrap();
                match receiver_lock.recv() {
                    Ok(msg) => msg,
                    Err(_) => break,
                }
            };
            println!("Got {}", msg);
        }
    });

    for thread in threads {
        thread.join().unwrap();
    }

    drop(sender_tx); // Drop the sender to signal the receiver to stop
    receiver_thread.join().unwrap();

    // Stop timer
    let duration: std::time::Duration = start.elapsed();
    println!(
        "took {}µs ({}ms) ({}s) to execute",
        duration.as_micros(),
        duration.as_secs_f64() * 1000.0,
        duration.as_secs_f64()
    );
}
*/
/*
Output 1:
Got This is thread ThreadId(2), message 1
Got This is thread ThreadId(3), message 11
Got This is thread ThreadId(4), message 21
Got This is thread ThreadId(2), message 2
Got This is thread ThreadId(2), message 3
Got This is thread ThreadId(4), message 22
Got This is thread ThreadId(3), message 12
Got This is thread ThreadId(2), message 4
Got This is thread ThreadId(4), message 23
Got This is thread ThreadId(3), message 13
Got This is thread ThreadId(2), message 5
Got This is thread ThreadId(4), message 24
Got This is thread ThreadId(3), message 14
Got This is thread ThreadId(2), message 6
Got This is thread ThreadId(3), message 15
Got This is thread ThreadId(4), message 25
Got This is thread ThreadId(2), message 7
Got This is thread ThreadId(3), message 16
Got This is thread ThreadId(4), message 26
Got This is thread ThreadId(2), message 8
Got This is thread ThreadId(3), message 17
Got This is thread ThreadId(4), message 27
Got This is thread ThreadId(2), message 9
Got This is thread ThreadId(3), message 18
Got This is thread ThreadId(2), message 10
Got This is thread ThreadId(4), message 28
Got This is thread ThreadId(3), message 19
Got This is thread ThreadId(4), message 29
Got This is thread ThreadId(3), message 20
Got This is thread ThreadId(4), message 30
took 396µs (0.396097ms) (0.000396097s) to execute
Output 2:
Got This is thread ThreadId(4), message 21
Got This is thread ThreadId(3), message 11
Got This is thread ThreadId(4), message 22
Got This is thread ThreadId(4), message 23
Got This is thread ThreadId(4), message 24
Got This is thread ThreadId(4), message 25
Got This is thread ThreadId(4), message 26
Got This is thread ThreadId(4), message 27
Got This is thread ThreadId(2), message 1
Got This is thread ThreadId(4), message 28
Got This is thread ThreadId(2), message 2
Got This is thread ThreadId(2), message 3
Got This is thread ThreadId(2), message 4
Got This is thread ThreadId(3), message 12
Got This is thread ThreadId(4), message 29
Got This is thread ThreadId(4), message 30
Got This is thread ThreadId(2), message 5
Got This is thread ThreadId(2), message 6
Got This is thread ThreadId(2), message 7
Got This is thread ThreadId(2), message 8
Got This is thread ThreadId(2), message 9
Got This is thread ThreadId(2), message 10
Got This is thread ThreadId(3), message 13
Got This is thread ThreadId(3), message 14
Got This is thread ThreadId(3), message 15
Got This is thread ThreadId(3), message 16
Got This is thread ThreadId(3), message 17
Got This is thread ThreadId(3), message 18
Got This is thread ThreadId(3), message 19
Got This is thread ThreadId(3), message 20
took 416µs (0.416673ms) (0.000416673s) to execute
Output 3:
Got This is thread ThreadId(2), message 1
Got This is thread ThreadId(4), message 21
Got This is thread ThreadId(3), message 11
Got This is thread ThreadId(4), message 22
Got This is thread ThreadId(2), message 2
Got This is thread ThreadId(4), message 23
Got This is thread ThreadId(2), message 3
Got This is thread ThreadId(4), message 24
Got This is thread ThreadId(2), message 4
Got This is thread ThreadId(4), message 25
Got This is thread ThreadId(2), message 5
Got This is thread ThreadId(4), message 26
Got This is thread ThreadId(2), message 6
Got This is thread ThreadId(4), message 27
Got This is thread ThreadId(2), message 7
Got This is thread ThreadId(4), message 28
Got This is thread ThreadId(3), message 12
Got This is thread ThreadId(2), message 8
Got This is thread ThreadId(4), message 29
Got This is thread ThreadId(3), message 13
Got This is thread ThreadId(2), message 9
Got This is thread ThreadId(4), message 30
Got This is thread ThreadId(2), message 10
Got This is thread ThreadId(3), message 14
Got This is thread ThreadId(3), message 15
Got This is thread ThreadId(3), message 16
Got This is thread ThreadId(3), message 17
Got This is thread ThreadId(3), message 18
Got This is thread ThreadId(3), message 19
Got This is thread ThreadId(3), message 20
took 358µs (0.358835ms) (0.000358835s) to execute
Output 4:
Got This is thread ThreadId(3), message 11
Got This is thread ThreadId(2), message 1
Got This is thread ThreadId(4), message 21
Got This is thread ThreadId(3), message 12
Got This is thread ThreadId(3), message 13
Got This is thread ThreadId(4), message 22
Got This is thread ThreadId(3), message 14
Got This is thread ThreadId(4), message 23
Got This is thread ThreadId(3), message 15
Got This is thread ThreadId(4), message 24
Got This is thread ThreadId(3), message 16
Got This is thread ThreadId(4), message 25
Got This is thread ThreadId(3), message 17
Got This is thread ThreadId(4), message 26
Got This is thread ThreadId(3), message 18
Got This is thread ThreadId(4), message 27
Got This is thread ThreadId(2), message 2
Got This is thread ThreadId(3), message 19
Got This is thread ThreadId(4), message 28
Got This is thread ThreadId(2), message 3
Got This is thread ThreadId(3), message 20
Got This is thread ThreadId(2), message 4
Got This is thread ThreadId(4), message 29
Got This is thread ThreadId(2), message 5
Got This is thread ThreadId(4), message 30
Got This is thread ThreadId(2), message 6
Got This is thread ThreadId(2), message 7
Got This is thread ThreadId(2), message 8
Got This is thread ThreadId(2), message 9
Got This is thread ThreadId(2), message 10
took 382µs (0.38275400000000004ms) (0.000382754s) to execute
*/




/*
// Now let's compare with this approach (our old):
// we indeed now use total of 4 threads (sender1, sender2, sender3, receiver)
use std::sync::mpsc;
use std::thread;

fn main() {
    let main_time: std::time::Instant = std::time::Instant::now();

    let (tx, rx) = mpsc::channel::<String>();
    let tx2 = tx.clone();
    let tx3 = tx.clone();

    let sender_thread1 = thread::spawn(move || {
        // println!("This is sender_thread, {:?}", std::thread::current().id());
        let tx: mpsc::Sender<String> = tx.clone();
        
        for i in 1..=10 {
            let val: String = format!("hi from task {}, thread {:?}", i, std::thread::current().id());
            match tx.send(val) {
                // Ok(_) => println!("Sent"),
                Ok(_) => {},
                Err(e) => {
                    // The channel is closed, so the receiver should exit
                    println!("Failed to send: {}", e);
                    // break;
                }
            };
        }
        // println!("sender_thread ({:?}) exit now...", std::thread::current().id());
    });
    
    let sender_thread2 = thread::spawn(move || {
        // println!("This is sender_thread, {:?}", std::thread::current().id());
        let tx: mpsc::Sender<String> = tx2.clone();
        
        for i in 11..=20 {
            let val: String = format!("hi from task {}, thread {:?}", i, std::thread::current().id());
            match tx.send(val) {
                // Ok(_) => println!("Sent"),
                Ok(_) => {},
                Err(e) => {
                    // The channel is closed, so the receiver should exit
                    println!("Failed to send: {}", e);
                    // break;
                }
            };
        }
        // println!("sender_thread ({:?}) exit now...", std::thread::current().id());
    });

    let sender_thread3 = thread::spawn(move || {
        // println!("This is sender_thread, {:?}", std::thread::current().id());
        let tx: mpsc::Sender<String> = tx3.clone();
        
        for i in 21..=30 {
            let val: String = format!("hi from task {}, thread {:?}", i, std::thread::current().id());
            match tx.send(val) {
                // Ok(_) => println!("Sent"),
                Ok(_) => {},
                Err(e) => {
                    // The channel is closed, so the receiver should exit
                    println!("Failed to send: {}", e);
                    // break;
                }
            };
        }
        // println!("sender_thread ({:?}) exit now...", std::thread::current().id());
    });


    // Spawn a separate thread to handle receiving & join the receiver thread
    let receiver_thread = thread::spawn(move || {
        // println!("This is receiver_thread");

        loop {
            match rx.recv() {
                Ok(mes) => println!("Got {}", mes),
                Err(_) => {
                    // The channel is closed, so the receiver should exit
                    println!("Channel closed. Nothing to receive, receiver exiting....");
                    break;
                }
            };
        }
        
        // println!("receiver_thread exit now...");
    });
    // must have
    receiver_thread.join();

    // End of main
    let duration: std::time::Duration = main_time.elapsed();
    println!(
        "took {}µs ({}ms) ({}s) to execute",
        duration.as_micros(),
        duration.as_secs_f64() * 1000.0,
        duration.as_secs_f64()
    );
}
*/
/*
Output 1:
Got hi from task 1, thread ThreadId(2)
Got hi from task 11, thread ThreadId(3)
Got hi from task 2, thread ThreadId(2)
Got hi from task 3, thread ThreadId(2)
Got hi from task 4, thread ThreadId(2)
Got hi from task 5, thread ThreadId(2)
Got hi from task 6, thread ThreadId(2)
Got hi from task 7, thread ThreadId(2)
Got hi from task 8, thread ThreadId(2)
Got hi from task 12, thread ThreadId(3)
Got hi from task 9, thread ThreadId(2)
Got hi from task 13, thread ThreadId(3)
Got hi from task 10, thread ThreadId(2)
Got hi from task 14, thread ThreadId(3)
Got hi from task 15, thread ThreadId(3)
Got hi from task 16, thread ThreadId(3)
Got hi from task 17, thread ThreadId(3)
Got hi from task 18, thread ThreadId(3)
Got hi from task 19, thread ThreadId(3)
Got hi from task 20, thread ThreadId(3)
Got hi from task 21, thread ThreadId(4)
Got hi from task 22, thread ThreadId(4)
Got hi from task 23, thread ThreadId(4)
Got hi from task 24, thread ThreadId(4)
Got hi from task 25, thread ThreadId(4)
Got hi from task 26, thread ThreadId(4)
Got hi from task 27, thread ThreadId(4)
Got hi from task 28, thread ThreadId(4)
Got hi from task 29, thread ThreadId(4)
Got hi from task 30, thread ThreadId(4)
Channel closed. Nothing to receive, receiver exiting....
took 394µs (0.39410999999999996ms) (0.00039411s) to execute
Output 2:
Got hi from task 1, thread ThreadId(2)
Got hi from task 21, thread ThreadId(4)
Got hi from task 11, thread ThreadId(3)
Got hi from task 2, thread ThreadId(2)
Got hi from task 3, thread ThreadId(2)
Got hi from task 4, thread ThreadId(2)
Got hi from task 5, thread ThreadId(2)
Got hi from task 6, thread ThreadId(2)
Got hi from task 7, thread ThreadId(2)
Got hi from task 22, thread ThreadId(4)
Got hi from task 8, thread ThreadId(2)
Got hi from task 12, thread ThreadId(3)
Got hi from task 9, thread ThreadId(2)
Got hi from task 10, thread ThreadId(2)
Got hi from task 23, thread ThreadId(4)
Got hi from task 24, thread ThreadId(4)
Got hi from task 25, thread ThreadId(4)
Got hi from task 26, thread ThreadId(4)
Got hi from task 27, thread ThreadId(4)
Got hi from task 13, thread ThreadId(3)
Got hi from task 28, thread ThreadId(4)
Got hi from task 14, thread ThreadId(3)
Got hi from task 29, thread ThreadId(4)
Got hi from task 15, thread ThreadId(3)
Got hi from task 30, thread ThreadId(4)
Got hi from task 16, thread ThreadId(3)
Got hi from task 17, thread ThreadId(3)
Got hi from task 18, thread ThreadId(3)
Got hi from task 19, thread ThreadId(3)
Got hi from task 20, thread ThreadId(3)
Channel closed. Nothing to receive, receiver exiting....
took 367µs (0.367687ms) (0.000367687s) to execute
Output 3:
Got hi from task 1, thread ThreadId(2)
Got hi from task 11, thread ThreadId(3)
Got hi from task 2, thread ThreadId(2)
Got hi from task 3, thread ThreadId(2)
Got hi from task 4, thread ThreadId(2)
Got hi from task 5, thread ThreadId(2)
Got hi from task 6, thread ThreadId(2)
Got hi from task 7, thread ThreadId(2)
Got hi from task 8, thread ThreadId(2)
Got hi from task 9, thread ThreadId(2)
Got hi from task 10, thread ThreadId(2)
Got hi from task 12, thread ThreadId(3)
Got hi from task 13, thread ThreadId(3)
Got hi from task 14, thread ThreadId(3)
Got hi from task 15, thread ThreadId(3)
Got hi from task 16, thread ThreadId(3)
Got hi from task 17, thread ThreadId(3)
Got hi from task 18, thread ThreadId(3)
Got hi from task 19, thread ThreadId(3)
Got hi from task 20, thread ThreadId(3)
Got hi from task 21, thread ThreadId(4)
Got hi from task 22, thread ThreadId(4)
Got hi from task 23, thread ThreadId(4)
Got hi from task 24, thread ThreadId(4)
Got hi from task 25, thread ThreadId(4)
Got hi from task 26, thread ThreadId(4)
Got hi from task 27, thread ThreadId(4)
Got hi from task 28, thread ThreadId(4)
Got hi from task 29, thread ThreadId(4)
Got hi from task 30, thread ThreadId(4)
Channel closed. Nothing to receive, receiver exiting....
took 845µs (0.845077ms) (0.000845077s) to execute
Output 4:
Got hi from task 11, thread ThreadId(3)
Got hi from task 1, thread ThreadId(2)
Got hi from task 12, thread ThreadId(3)
Got hi from task 13, thread ThreadId(3)
Got hi from task 14, thread ThreadId(3)
Got hi from task 21, thread ThreadId(4)
Got hi from task 22, thread ThreadId(4)
Got hi from task 23, thread ThreadId(4)
Got hi from task 24, thread ThreadId(4)
Got hi from task 25, thread ThreadId(4)
Got hi from task 26, thread ThreadId(4)
Got hi from task 27, thread ThreadId(4)
Got hi from task 28, thread ThreadId(4)
Got hi from task 29, thread ThreadId(4)
Got hi from task 30, thread ThreadId(4)
Got hi from task 15, thread ThreadId(3)
Got hi from task 16, thread ThreadId(3)
Got hi from task 17, thread ThreadId(3)
Got hi from task 18, thread ThreadId(3)
Got hi from task 19, thread ThreadId(3)
Got hi from task 20, thread ThreadId(3)
Got hi from task 2, thread ThreadId(2)
Got hi from task 3, thread ThreadId(2)
Got hi from task 4, thread ThreadId(2)
Got hi from task 5, thread ThreadId(2)
Got hi from task 6, thread ThreadId(2)
Got hi from task 7, thread ThreadId(2)
Got hi from task 8, thread ThreadId(2)
Got hi from task 9, thread ThreadId(2)
Got hi from task 10, thread ThreadId(2)
Channel closed. Nothing to receive, receiver exiting....
took 396µs (0.39621300000000004ms) (0.000396213s) to execute
*/




/*
// putting thread::spawn() inside a for() loop is bad.
// Conclusion, spawning N threads for N tasks is a really bad approach for performance & control flow
use std::sync::{mpsc, Arc, Mutex};
use std::thread;


fn main() {
    let main_time: std::time::Instant = std::time::Instant::now();

    let mut handles = vec![];

    let (tx, rx) = mpsc::channel::<String>();

    // does not terminates
    // Spawn threads to send messages
    for i in 1..=3 {
        let tx = tx.clone();
        let join_handle = thread::spawn(move || {

            let val = format!("hi from thread {}", i);
            if let Err(_) = tx.send(val) {
                println!("failed to sent, reciver dropped");
                return;
            }
            println!("sent");

        });
        handles.push(join_handle);
    }

    for h in handles {
        h.join().unwrap();
    }


    // Approach 3: Spawn a separate thread to handle receiving & join the receiver thread
    let receiver_thread = thread::spawn(move || {
        // println!("This is receiver_thread");
        // Receive messages and print them as they arrive
        // while let Ok(msg) = rx.recv() {
        //     println!("Got {}", msg);
        // }

        loop {
            match rx.recv() {
                Ok(mes) => println!("Got {}", mes),
                Err(_) => {
                    // The channel is closed, so the receiver should exit
                    println!("Channel closed. Nothing to receive, receiver exiting....");
                    break;
                }
            };
        }
        
        // println!("receiver_thread exit now...");
    });

    
    // mutual thread, does not terminates
    // loop {
    //     match rx.recv() {
    //         Ok(mes) => println!("Got {}", mes),
    //         Err(_) => {
    //             // The channel is closed, so the receiver should exit
    //             println!("Channel closed. Nothing to receive, receiver exiting....");
    //             break;
    //         }
    //     };
    // }

    // receiver_thread.join();
    // sender_thread.join();    // doesn't really needed to

    // works
    // while let Ok(msg) = rx.recv() {
    //     println!("Got {}", msg);
    // }

    // End of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\n⌛️ Execution time: {:?} ({:?} ms)", duration, elapsed_ms);
    
}
*/
/*
Output:
sent
sent
sent

⌛️ Execution time: 376.039µs (0.376039 ms)
Got hi from thread 1
Got hi from thread 2
Got hi from thread 3
Channel closed. Nothing to receive, receiver exiting....
*/




/*
// our original worker_pool mpsc example:
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;

const NUM_WORKERS: usize = 4;
const NUM_TASKS: usize = 20;

fn main() {
    let main_time = std::time::Instant::now();

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
                // thread::sleep(std::time::Duration::from_secs(1));

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

    // End of main
    let duration: std::time::Duration = main_time.elapsed();
    println!(
        "took {}µs ({}ms) ({}s) to execute",
        duration.as_micros(),
        duration.as_secs_f64() * 1000.0,
        duration.as_secs_f64()
    );
}
*/
/*
Output:
Worker 0 is waiting for tasks.
Worker 0 is processing task: 0
Worker 3 is waiting for tasks.
Worker 0 is processing task: 1
Worker 0 is processing task: 3
Worker 0 is processing task: 4
Worker 0 is processing task: 5
Worker 0 is processing task: 6
Worker 0 is processing task: 7
Worker 0 is processing task: 8
Worker 0 is processing task: 9
Worker 0 is processing task: 10
Worker 0 is processing task: 11
Worker 0 is processing task: 12
Worker 0 is processing task: 13
Worker 0 is processing task: 14
Worker 0 is processing task: 15
Worker 0 is processing task: 16
Worker 0 is processing task: 17
Worker 0 is processing task: 18
Worker 0 is processing task: 19
Worker 0 is exiting.
Worker 1 is waiting for tasks.
Worker 1 is exiting.
Worker 2 is waiting for tasks.
Worker 2 is exiting.
Worker 3 is processing task: 2
Worker 3 is exiting.
Received result: Result of task 0: done
Received result: Result of task 1: done
Received result: Result of task 3: done
Received result: Result of task 4: done
Received result: Result of task 5: done
Received result: Result of task 6: done
Received result: Result of task 7: done
Received result: Result of task 8: done
Received result: Result of task 9: done
Received result: Result of task 10: done
Received result: Result of task 11: done
Received result: Result of task 12: done
Received result: Result of task 13: done
Received result: Result of task 14: done
Received result: Result of task 15: done
Received result: Result of task 16: done
Received result: Result of task 17: done
Received result: Result of task 18: done
Received result: Result of task 19: done
Received result: Result of task 2: done
took 558µs (0.5587040000000001ms) (0.000558704s) to execute
*/





/*
// trying react's websocket communication
// main.rs
use tokio::net::TcpListener;
use tokio_tungstenite::*;
use futures::StreamExt;
// use tungstenite::Message;
use tungstenite::protocol::Message as TungsteniteMessage;
use tungstenite::Error;
use futures::SinkExt;


#[derive(Debug)]
enum Message {
    Text(String),
    Binary(Vec<u8>),
}

impl From<TungsteniteMessage> for Message {
    fn from(msg: TungsteniteMessage) -> Self {
        match msg {
            TungsteniteMessage::Text(text) => Message::Text(text),
            TungsteniteMessage::Binary(bin) => Message::Binary(bin),
            _ => {
                println!("Unsupported message type: {:?}", msg);
                Message::Text(String::from("Unsupported message type"))
            }
        }
    }
}

impl Into<TungsteniteMessage> for Message {
    fn into(self) -> TungsteniteMessage {
        match self {
            Message::Text(text) => TungsteniteMessage::Text(text),
            Message::Binary(bin) => TungsteniteMessage::Binary(bin),
        }
    }
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");

    println!("Server running on {}", addr);

    // loop {
    //     let (stream, _) = listener.accept().await.expect("Failed to accept");
    //     tokio::spawn(handle_connection(stream));
    // }
    while let Ok((stream, client_addr)) = listener.accept().await {
        tokio::spawn(handle_connection(stream, client_addr));
    }
}

async fn handle_connection(stream: tokio::net::TcpStream, client_addr: std::net::SocketAddr) {
    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during WebSocket handshake");

    let (tx, rx) = futures_channel::mpsc::unbounded();
    // let (write, read) = ws_stream.split();
    let (write, read) = ws_stream.split();

    tokio::spawn(async {
        read.for_each(|msg| async {
            match msg {
                Ok(msg) => {
                    let message: Message = msg.into();
                    println!("Received message: {:?}", message);
                    // Handle incoming message here
                    let response = format!("Hi {}", client_addr);
                    write.send(Message::Text(response).into()).await.unwrap();
                }
                Err(Error::ConnectionClosed) => {
                    println!("Connection closed");
                }
                Err(e) => {
                    println!("Error receiving message: {}", e);
                }
            }
        }).await;
    });

    // Example of sending a message back to the client
    // write.send(Message::Text("Hello from Rust!".into())).await.unwrap();
}

*/

/*
// status: WORKED
// implemented: connection accepting, messaging, send back to sender
use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, StreamExt, SinkExt, TryStreamExt};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<std::net::SocketAddr, Tx>>>;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Failed to bind");
    let peer_map: PeerMap = Arc::new(Mutex::new(HashMap::new()));

    println!("WebSocket server is running on ws://127.0.0.1:8080");

    while let Ok((stream, addr)) = listener.accept().await {
        let peer_map_clone = peer_map.clone();
        tokio::spawn(handle_connection(stream, addr, peer_map_clone));
    }
}

async fn handle_connection(stream: TcpStream, addr: std::net::SocketAddr, peer_map: PeerMap) {
    if let Ok(ws_stream) = accept_async(stream).await {
        println!("Client connected from: {}", addr);

        let (tx, rx) = unbounded();

        // Insert the sender into the PeerMap
        peer_map.lock().unwrap().insert(addr, tx.clone());

        let (write, read) = ws_stream.split();

        let broadcast_incoming = read.try_for_each(|msg| {
            println!("Received message from {}: {}", addr, msg.to_text().unwrap());

            let peers = peer_map.lock().unwrap();

            // Broadcast message to everyone except the sender.
            let broadcast_recipients = peers
                .iter()
                .filter(|(peer_addr, _)| peer_addr != &&addr)
                .map(|(_, ws_sink)| ws_sink);

            for recp in broadcast_recipients {
                recp.unbounded_send(msg.clone()).unwrap();
            }

            // Send message back to the sender with a greeting
            let sender_tx = peers.get(&addr).unwrap();
            let greeting_message = format!("Hi client {}!", addr);
            sender_tx.unbounded_send(Message::Text(greeting_message)).unwrap();

            future::ok(())
        });

        let receive_from_others = rx.map(Ok).forward(write);

        futures_util::pin_mut!(broadcast_incoming, receive_from_others);
        future::select(broadcast_incoming, receive_from_others).await;

        println!("{} disconnected", &addr);
        peer_map.lock().unwrap().remove(&addr);
    }
}
*/




/*
// status: not receiving message or response back
// trying to implement welcome message
use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, StreamExt, SinkExt, TryStreamExt};
use tokio::sync::Mutex;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
// use std::sync::{Arc, Mutex};    // => future cannot be sent between threads safely
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Failed to bind");
    let peer_map: PeerMap = Arc::new(Mutex::new(HashMap::new()));
    let peer_map_clone = Arc::clone(&peer_map);

    println!("WebSocket server is running on ws://127.0.0.1:8080");

    while let Ok((stream, addr)) = listener.accept().await {
        let peer_map_clone = peer_map.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, addr, &peer_map_clone).await {
                eprintln!("Error handling connection: {}", e);
            }
        });
    }
}

// added "&" => &PeerMap to fix "cannot move out of captured outer ...""
async fn handle_connection(
    stream: TcpStream,
    addr: SocketAddr,
    peer_map: &PeerMap,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let ws_stream = accept_async(stream).await?;

    println!("Client connected from: {}", addr);

    // Send welcome message to all peer clients
    // let peers = peer_map.lock().unwrap();   // std::sync::Mutex
    let peers = peer_map.lock().await;
    let welcome_message = format!("Welcome client {} to the WebSocket server", addr);
    for (_, tx) in peers.iter() {
        tx.unbounded_send(Message::Text(welcome_message.clone()))?;
    }

    let (tx, rx) = unbounded();
    // let mut peers = peer_map.lock().unwrap();    // std::sync::Mutex
    let mut peers = peer_map.lock().await;
    peers.insert(addr, tx.clone());

    let (write, read) = ws_stream.split();

    /*
    let broadcast_incoming = read.try_for_each(|msg| {
        println!("Received message from {}: {}", addr, msg.to_text().unwrap());

        let peers = peer_map.lock().unwrap();

        let broadcast_recipients = peers
            .iter()
            .filter(|(peer_addr, _)| peer_addr != &&addr)
            .map(|(_, ws_sink)| ws_sink);

        for recp in broadcast_recipients {
            recp.unbounded_send(msg.clone())?;
        }

        Ok(())
    });
    */

    let broadcast_incoming = read.try_for_each(|msg| {
        async move {
            println!("Received message from {}: {}", addr, msg.to_text().unwrap());
    
            // let peers = peer_map.lock().unwrap();   // std::sync::Mutex
            let peers = peer_map.lock().await;
    
            let broadcast_recipients = peers
                .iter()
                .filter(|(peer_addr, _)| peer_addr != &&addr)
                .map(|(_, ws_sink)| ws_sink);
    
            for recp in broadcast_recipients {
                if let Err(err) = recp.unbounded_send(msg.clone()) {
                    eprintln!("Error sending message: {}", err);
                }
            }

            // Send message back to the sender with a greeting
            let sender_tx = peers.get(&addr).unwrap();
            let greeting_message = format!("Hi client {}!", addr);
            sender_tx.unbounded_send(Message::Text(greeting_message)).unwrap();

            // future::ok(())
    
            Ok(())
        }
    });

    let receive_from_others = rx.map(Ok).forward(write);

        futures_util::pin_mut!(broadcast_incoming, receive_from_others);
        future::select(broadcast_incoming, receive_from_others).await;

        println!("{} disconnected", &addr);
        peer_map.lock().await.remove(&addr);

    // let receive_from_others = rx.map(Ok).forward(write);

    // broadcast_incoming.await?;
    // receive_from_others.await?;

    println!("{} disconnected", &addr);
    peers.remove(&addr);

    Ok(())
}
*/



/*
// still fails
use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{SinkExt, StreamExt, TryStreamExt};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio::sync::Mutex;
use futures_util::future::{Either, BoxFuture};
use futures_util::FutureExt;

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Failed to bind");
    let peer_map: PeerMap = Arc::new(Mutex::new(HashMap::new()));

    println!("WebSocket server is running on ws://127.0.0.1:8080");

    while let Ok((stream, addr)) = listener.accept().await {
        let peer_map = Arc::clone(&peer_map);
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, addr, peer_map).await {
                eprintln!("Error handling connection: {}", e);
            }
        });
    }
}

async fn handle_connection(
    stream: TcpStream,
    addr: SocketAddr,
    peer_map: PeerMap,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let ws_stream = accept_async(stream).await?;

    println!("Client connected from: {}", addr);

    // Send welcome message to the client
    let welcome_message = format!("Welcome client {} to the WebSocket server", addr);
    send_message(&peer_map, &addr, Message::Text(welcome_message)).await?;

    let (tx, rx) = unbounded();
    peer_map.lock().await.insert(addr, tx);

    let (write, read) = ws_stream.split();

    let incoming_messages = read.try_for_each(|msg| async {
        println!("Received message from {}: {}", addr, msg);

        // Broadcast the message to all other clients
        let peer_map = peer_map.lock().await;
        let broadcast_recipients: Vec<_> = peer_map.iter()
            .filter(|(peer_addr, _)| **peer_addr != addr)
            .map(|(_, tx)| tx.send(msg.clone()))
            .collect();

        for send_result in broadcast_recipients {
            if let Err(e) = send_result.await {
                eprintln!("Error sending message to client: {}", e);
            }
        }

        Ok(())
    });

    let outgoing_messages = rx.map(Ok).forward(write);

    let result = futures_util::future::select(
        incoming_messages.boxed::<Box<dyn std::error::Error + Send + Sync>>(), 
        outgoing_messages.boxed::<Box<dyn std::error::Error + Send + Sync>>()
    ).await;

    match result {
        Either::Left((err, _)) => {
            eprintln!("Error handling WebSocket connection with client {}: {:?}", addr, err);
        }
        Either::Right((err, _)) => {
            eprintln!("Error handling WebSocket connection with client {}: {:?}", addr, err);
        }
    }

    peer_map.lock().await.remove(&addr);
    println!("Client disconnected: {}", addr);

    Ok(())
}

async fn send_message(peer_map: &PeerMap, addr: &SocketAddr, message: Message) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut peer_map = peer_map.lock().await;
    if let Some(tx) = peer_map.get_mut(addr) {
        tx.unbounded_send(message)?;
    } else {
        return Err("Client not found in peer map".into());
    }
    Ok(())
}
*/




/*
// PhantomData example in type infering practice for generic repository pattern desire
use std::marker::PhantomData;

// Define a custom type
struct User;

// Define a trait to get the type name
trait TypeName {
    fn type_name() -> &'static str;
}

// Implement the TypeName trait for the User type
impl TypeName for User {
    fn type_name() -> &'static str {
        "User"
    }
}

// Define a generic struct with a PhantomData field
struct MyGenericStruct<T> {
    phantom: PhantomData<T>,
}

// Implement a function to print the type name
fn print_type_name<T: TypeName>() {
    println!("Type name: {}", T::type_name());
}

fn main() {
    // Create an instance of MyGenericStruct with User as the type parameter
    let my_user = MyGenericStruct::<User> { phantom: PhantomData };

    // Call the print_type_name function with User as the type parameter
    print_type_name::<User>();
}
*/



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
// 1 error remaining:
error[E0599]: no method named `lock` found for mutable reference `&mut HashMap<usize, SplitSink<WebSocketStream<tokio::net::TcpStream>, Message>>` in the current scope
    --> learning/src/main.rs:6124:55
     |
6124 | ...       if let Err(e) = clients.get_mut().lock().await[&client_id].send(Message::...
     |                           -------           ^^^^ method not found in `&mut HashMap<usize, SplitSink<WebSocketStream<TcpStream>, Message>>`
     |                           |
     |                           method `lock` is available on `&mut tokio::sync::Mutex<HashMap<usize, SplitSink<WebSocketStream<tokio::net::TcpStream>, Message>>>`

For more information about this error, try `rustc --explain E0599`.
warning: `learning` (bin "learning") generated 1 warning
error: could not compile `learning` (bin "learning") due to 1 previous error; 1 warning emitted
*/

// Explanatory (for previous code with 3 errors): It seems we have mistakenly used lock() on Arc instances directly, which is incorrect. We should first dereference Arc to access the HashMap and then use lock() on the resulting Mutex. Let's correct that:
/*
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
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
// Final solution for tokio_tungstenite todo

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


#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
enum PrintableMessage {
    Text(String),
    Binary(Vec<u8>),
    // Add more variants as needed
}

impl From<Message> for PrintableMessage {
    fn from(msg: Message) -> Self {
        match msg {
            Message::Text(text) => PrintableMessage::Text(text),
            Message::Binary(bin_data) => PrintableMessage::Binary(bin_data),
            // Handle more variants if necessary
            _ => unimplemented!(), // Ignore other variants
            // will print "thread 'tokio-runtime-worker' panicked at learning/src/main.rs:6232:18: not implemented" when a client disconnect
        }
    }
}

impl Into<Message> for PrintableMessage {
    fn into(self) -> Message {
        match self {
            PrintableMessage::Text(text) => Message::Text(text),
            PrintableMessage::Binary(bin_data) => Message::Binary(bin_data),
            // Handle more variants if necessary
        }
    }
}


#[tokio::main]
async fn main() {

    let addr = "127.0.0.1:8080";
    let listener: TcpListener = TcpListener::bind(addr).await.expect("Failed to bind");
    let clients: Arc<Mutex<HashMap<usize, SplitSink<tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>, Message>>>> = Arc::new(Mutex::new(HashMap::new()));
    let tasks: Arc<Mutex<HashMap<usize, Task>>> = Arc::new(Mutex::new(HashMap::new()));
    let task_id: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));

    loop {
        let (stream, peer_socket_addr) = listener.accept().await.expect("Failed to accept");
        println!("accepted a connection from: {}", peer_socket_addr);
        let clients: Arc<Mutex<HashMap<usize, SplitSink<tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>, Message>>>> = Arc::clone(&clients);
        let tasks: Arc<Mutex<HashMap<usize, Task>>> = Arc::clone(&tasks);
        let task_id: Arc<Mutex<usize>> = Arc::clone(&task_id);

        tokio::spawn(async move {

            let ws_stream: tokio_tungstenite::WebSocketStream<tokio::net::TcpStream> = accept_async(stream).await.expect("Error during WebSocket handshake");
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

                        let printable_msg: PrintableMessage = msg.clone().into();
                        println!("Received a message: {}", serde_json::to_string_pretty(&printable_msg).unwrap());
            
                        match msg {
                            Message::Text(text) => {

                                // Attempt to deserialize the text into a Task object
                                if let Ok(mut received_task) = serde_json::from_str::<Task>(&text) {

                                    println!("message is Text: {:?}", received_task);
                                    println!("Json pretty:\n{}", serde_json::to_string_pretty(&received_task).unwrap());

                                    let mut tasks: tokio::sync::MutexGuard<HashMap<usize, Task>> = tasks.lock().await;
                                    
                                    match received_task.status.as_str() {
                                        "create" => {
                                            println!("create field detected");
                                            let mut task_id: tokio::sync::MutexGuard<usize> = task_id.lock().await;
                                            *task_id += 1;
                                            received_task.id = *task_id;
                                            tasks.insert(received_task.id, received_task.clone());
                                        }
                                        "update" => {
                                            println!("update field detected");
                                            if tasks.contains_key(&received_task.id) {
                                                tasks.insert(received_task.id, received_task.clone());
                                            }
                                        }
                                        "delete" => {
                                            println!("delete field detected");
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
                                } else {
                                    // Handle the case where the text cannot be deserialized into a Task object
                                    println!("Received invalid JSON: {}", text);
                                }
                            }
                            _ => {
                                if let Ok(printable_msg) = serde_json::from_str::<PrintableMessage>(&msg.to_string()) {
                                    println!("Received a printable message: {:?}", printable_msg);
                                } else {
                                    println!("Received non-text message");
                                }
                                // Handle other message types if needed
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error receiving message: {:?}", e);
                        break;
                    }
                }
            }
            
            
            let mut clients: tokio::sync::MutexGuard<HashMap<usize, SplitSink<tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>, Message>>> = clients.lock().await;
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




/*
// random number generate
// status: worked
use rand::Rng;
use tokio_tungstenite::tungstenite::Message;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::{broadcast, Mutex};
use tokio_tungstenite::accept_async;
use futures::SinkExt;   // fix error: no method named `send` found for struct `SplitSink` in the current scope
// use tungstenite::protocol::Message;

#[tokio::main]
async fn main() {
    // Create a TCP listener
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Server running on {}", addr);

    // Create a broadcast channel to send messages to all clients
    let (tx, _) = broadcast::channel::<Message>(10);
    let tx = Arc::new(Mutex::new(tx));

    loop {
        let (stream, peer_socket_addr) = listener.accept().await.unwrap();
        println!("accepted a connection from {}", peer_socket_addr);
        let tx = Arc::clone(&tx);
        
        // Spawn a new task for each incoming connection
        tokio::spawn(handle_connection(stream, tx));
    }
}

async fn handle_connection(stream: tokio::net::TcpStream, tx: Arc<Mutex<broadcast::Sender<Message>>>) {
    let ws_stream = accept_async(stream)
        .await
        .expect("Error during websocket handshake");

    let (mut write, _) = futures::StreamExt::split(ws_stream);

    // Clone the broadcast sender to be used in this connection
    let tx = Arc::clone(&tx);

    // Spawn a task to continuously send random numbers to this client
    tokio::spawn(async move {
        loop {
            /*
            // send just a number: eg. 443
            let random_number = rand::thread_rng().gen_range(100..=1000);
            let message = Message::Text(random_number.to_string());
            
            // Send the random number to this client
            if let Err(_) = write.send(message.clone()).await {
                // If sending fails, the client has disconnected, so break out of the loop
                break;
            }
            */
            // send json format: eg. {"number":443}
            let random_number = rand::thread_rng().gen_range(1000..=10000);
            let json_message = serde_json::json!({"number": random_number});
            let message = Message::Text(json_message.to_string());
            
            // Send the JSON message to this client
            if let Err(_) = write.send(message.clone()).await {
                // If sending fails, the client has disconnected, so break out of the loop
                break;
            }

            // Broadcast the message to all connected clients
            let mut tx = tx.lock().await;

            match tx.send(message.clone()) {
                Ok(bytes) => {} // ignore if succeeded
                // Err(err) => println!("err sending to connected clients: {:?}", err)
                Err(_) => {}    // ignore if fails
            }
            
            // Delay for a short interval before sending the next message
            // Or just no delay needed
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    });
}
*/



/*
// random color hex code
use futures::StreamExt; // for ws_stream.split();
use futures::SinkExt;   // for write.send(message.clone()).await
use rand::Rng;
use serde_json::{json, Value};
use tokio_tungstenite::tungstenite::Message;
// use std::sync::{Arc, Mutex}; // => error: future cannot be sent between threads safely
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio_tungstenite::accept_async;

#[tokio::main]
async fn main() {
    // Create a TCP listener
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Server running on {}", addr);

    // Create a broadcast channel to send messages to all clients
    let (tx, _) = broadcast::channel::<Message>(10);
    let tx = Arc::new(Mutex::new(tx));

    loop {
        let (stream, peer_socket_addr) = listener.accept().await.unwrap();
        println!("accepted a connection from: {}", peer_socket_addr);
        let tx = Arc::clone(&tx);
        
        // Spawn a new task for each incoming connection
        tokio::spawn(handle_connection(stream, tx));
    }
}

async fn handle_connection(stream: tokio::net::TcpStream, tx: Arc<Mutex<broadcast::Sender<Message>>>) {
    let ws_stream = accept_async(stream)
        .await
        .expect("Error during websocket handshake");

    let (mut write, _) = ws_stream.split();

    // Clone the broadcast sender to be used in this connection
    let tx = Arc::clone(&tx);

    // Spawn a task to continuously send random color hex codes to this client
    tokio::spawn(async move {
        loop {
            let random_color = generate_random_color();
            let json_message = json!({"color": random_color});
            let message = Message::Text(json_message.to_string());
            
            // Send the JSON message to this client
            if let Err(_) = write.send(message.clone()).await {
                // If sending fails, the client has disconnected, so break out of the loop
                break;
            }

            // Broadcast the message to all connected clients
            let mut tx = tx.lock().await;

            // tx.send(message.clone()).unwrap();
            match tx.send(message.clone()) {
                Ok(_) => {} // ignore if succeeded
                Err(err) => println!("err sending to connected clients: {:?}", err)
            };

            // match tx.send(message.clone()) {
            //     Ok(_) => {} // ignore if succeeded
            //     Err(broadcast::error::SendError::Closed(_)) => {
            //         // The receiver has been closed, likely due to client disconnection
            //         println!("Failed to send message: channel closed");
            //     }
            //     Err(broadcast::error::SendError::Lagged(_)) => {
            //         // The message could not be sent because the receiver is lagged
            //         println!("Failed to send message: channel lagged");
            //     }
            // };
            
            // Delay for a short interval before sending the next message
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    });
}

fn generate_random_color() -> String {
    let mut rng = rand::thread_rng();
    let r: u8 = rng.gen_range(0..=255);
    let g: u8 = rng.gen_range(0..=255);
    let b: u8 = rng.gen_range(0..=255);
    format!("{:02X}{:02X}{:02X}", r, g, b)
}
*/




// try to fix the mismatch issue of different messages sent to different connected clients.
/*
use futures::StreamExt; // for ws_stream.split();
use futures::SinkExt;   // for write.send(message.clone()).await
use rand::Rng;
use serde_json::{json, Value};
use tokio_tungstenite::tungstenite::Message;
// use std::sync::{Arc, Mutex}; // => error: future cannot be sent between threads safely
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio_tungstenite::accept_async;

#[tokio::main]
async fn main() {
    // Create a TCP listener
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await.expect("Failed to bind address");
    println!("Server running on {}", addr);

    // Create a broadcast channel to send messages to all clients
    let (tx, _) = broadcast::channel::<Message>(10);
    let tx = Arc::new(Mutex::new(tx));

    loop {
        let (stream, _) = listener.accept().await.expect("Failed to accept connection");
        let tx = Arc::clone(&tx);
        
        // Spawn a new task for each incoming connection
        tokio::spawn(handle_connection(stream, tx));
    }
}

async fn handle_connection(stream: tokio::net::TcpStream, tx: Arc<Mutex<broadcast::Sender<Message>>>) {
    let ws_stream = accept_async(stream)
        .await
        .expect("Error during websocket handshake");

    let (mut write, _) = ws_stream.split();

    // Clone the broadcast sender to be used in this connection
    let tx = Arc::clone(&tx);

    // Spawn a task to continuously send random color hex codes to this client
    tokio::spawn(async move {
        loop {
            let random_color = generate_random_color();
            let json_message = json!({"color": random_color});
            let message = Message::Text(json_message.to_string());
            
            // Broadcast the message to all connected clients
            let mut tx = tx.lock().await;
            match tx.send(message.clone()) {
                Ok(_) => {} // ignore if succeeded
                Err(broadcast::error::SendError::Closed(_)) => {
                    // The receiver has been closed, likely due to client disconnection
                    println!("Failed to send message: channel closed");
                }
                Err(broadcast::error::SendError::Lagged(_)) => {
                    // The message could not be sent because the receiver is lagged
                    println!("Failed to send message: channel lagged");
                }
            };
            
            // Delay for a short interval before sending the next message
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    });
}

fn generate_random_color() -> String {
    let mut rng = rand::thread_rng();
    let r: u8 = rng.gen_range(0..=255);
    let g: u8 = rng.gen_range(0..=255);
    let b: u8 = rng.gen_range(0..=255);
    format!("{:02X}{:02X}{:02X}", r, g, b)
}
*/




/*
// future cannot be sent between threads safely
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use futures_util::StreamExt;
use futures_util::SinkExt;
use std::sync::Arc;
use rand::Rng;
use std::time::Duration;

type Tx = tokio::sync::mpsc::UnboundedSender<Message>;
type PeerList = Arc<Mutex<Vec<Tx>>>;

async fn handle_connection(peer_list: PeerList, stream: tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>) {
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    {
        let mut peers = peer_list.lock().await;
        peers.push(tx);
    }

    let (mut ws_sender, mut ws_receiver) = stream.split();

    let send_task = tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            if ws_sender.send(message).await.is_err() {
                break;
            }
        }
    });

    let receive_task = tokio::spawn(async move {
        while ws_receiver.next().await.is_some() {}
    });

    tokio::select! {
        _ = send_task => {}
        _ = receive_task => {}
    }

    let mut peers = peer_list.lock().await;
    if let Some(index) = peers.iter().position(|x| x.is_closed()) {
        peers.remove(index);
    }
}

async fn broadcast_random_numbers(peer_list: PeerList) {
    let mut interval = tokio::time::interval(Duration::from_secs(1));
    let mut rng = rand::thread_rng();

    loop {
        interval.tick().await;
        let number = rng.gen_range(1..=10);
        let message = Message::text(number.to_string());

        let peers = peer_list.lock().await;
        for tx in peers.iter() {
            let _ = tx.send(message.clone());
        }
    }
}

#[tokio::main]
async fn main() {
    let peer_list: PeerList = Arc::new(Mutex::new(Vec::new()));
    let listener = TcpListener::bind("127.0.0.1:8080").await.expect("Failed to bind");

    let peer_list_clone = peer_list.clone();
    tokio::spawn(async move {
        broadcast_random_numbers(peer_list_clone).await;
    });

    while let Ok((stream, _)) = listener.accept().await {
        let peer_list_clone = peer_list.clone();
        tokio::spawn(async move {
            if let Ok(ws_stream) = accept_async(stream).await {
                handle_connection(peer_list_clone, ws_stream).await;
            }
        });
    }
}
*/




/*
// fixed "future cannot be sent between threads safely"
// explanatory: 
use tokio::net::TcpListener;
use tokio::sync::{mpsc, Mutex};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use futures_util::StreamExt;
use futures_util::SinkExt;
use std::sync::Arc;
use std::time::Duration;
use rand::Rng;


type Tx = mpsc::UnboundedSender<Message>;
type PeerList = Arc<Mutex<Vec<Tx>>>;

async fn handle_connection(peer_list: PeerList, stream: tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>) {
    let (tx, mut rx) = mpsc::unbounded_channel();
    {
        let mut peers = peer_list.lock().await;
        peers.push(tx);
    }

    let (mut ws_sender, mut ws_receiver) = stream.split();

    let send_task = tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            if ws_sender.send(message).await.is_err() {
                break;
            }
        }
    });

    let receive_task = tokio::spawn(async move {
        while ws_receiver.next().await.is_some() {}
    });

    tokio::select! {
        _ = send_task => {}
        _ = receive_task => {}
    }

    let mut peers = peer_list.lock().await;
    if let Some(index) = peers.iter().position(|x| x.is_closed()) {
        peers.remove(index);
    }
}

async fn broadcast_random_numbers(peer_list: PeerList, mut number_rx: mpsc::UnboundedReceiver<i32>) {
    while let Some(number) = number_rx.recv().await {
        let message = Message::text(number.to_string());

        let peers = peer_list.lock().await;
        for tx in peers.iter() {
            let _ = tx.send(message.clone());
        }
    }
}

async fn generate_random_numbers(number_tx: mpsc::UnboundedSender<i32>) {
    let mut interval = tokio::time::interval(Duration::from_secs(1));

    loop {
        interval.tick().await;
        let number = rand::thread_rng().gen_range(1..=10);
        let _ = number_tx.send(number);
    }
}

#[tokio::main]
async fn main() {
    let peer_list: PeerList = Arc::new(Mutex::new(Vec::new()));
    let listener = TcpListener::bind("127.0.0.1:8080").await.expect("Failed to bind");

    let (number_tx, number_rx) = mpsc::unbounded_channel();
    let peer_list_clone = peer_list.clone();

    tokio::spawn(async move {
        generate_random_numbers(number_tx).await;
    });

    tokio::spawn(async move {
        broadcast_random_numbers(peer_list_clone, number_rx).await;
    });

    while let Ok((stream, _)) = listener.accept().await {
        let peer_list_clone = peer_list.clone();
        tokio::spawn(async move {
            if let Ok(ws_stream) = accept_async(stream).await {
                handle_connection(peer_list_clone, ws_stream).await;
            }
        });
    }
}
*/



// STATUS: worked flawlessly
// features: broadcast generated number, clients connection/disconnection event
/*
use tokio::net::TcpListener;
use tokio::sync::{mpsc, Mutex};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use futures_util::StreamExt;
use futures_util::SinkExt;
use std::sync::Arc;
use std::time::Duration;
use rand::Rng;

type Tx = mpsc::UnboundedSender<Message>;
type PeerList = Arc<Mutex<Vec<Tx>>>;

async fn handle_connection(peer_list: PeerList, stream: tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>, peer_addr: std::net::SocketAddr) {
    let (tx, mut rx) = mpsc::unbounded_channel();
    {
        let mut peers: tokio::sync::MutexGuard<Vec<mpsc::UnboundedSender<Message>>> = peer_list.lock().await;
        peers.push(tx.clone());
        broadcast_client_count(&peers).await;
        println!("Client connected: {}", peer_addr);
        println!("Current peers: {}", peers.len());
    }

    let (mut ws_sender, mut ws_receiver) = stream.split();

    let send_task = tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            if ws_sender.send(message).await.is_err() {
                break;
            }
        }
    });

    let receive_task: tokio::task::JoinHandle<()> = tokio::spawn(async move {
        while ws_receiver.next().await.is_some() {}
    });

    tokio::select! {
        _ = send_task => {}
        _ = receive_task => {}
    }

    {
        let mut peers: tokio::sync::MutexGuard<Vec<mpsc::UnboundedSender<Message>>> = peer_list.lock().await;
        if let Some(index) = peers.iter().position(|peer_tx| peer_tx.same_channel(&tx)) {
            peers.remove(index);
            broadcast_client_count(&peers).await;
            println!("Client disconnected: {}", peer_addr);
            println!("Current peers: {}", peers.len());
        }
    }
}

async fn broadcast_random_numbers(peer_list: PeerList, mut number_rx: mpsc::UnboundedReceiver<i32>) {
    while let Some(number) = number_rx.recv().await {
        // let message = Message::text(number.to_string());
        let json_message = serde_json::json!({"number": number.to_string()});
        let message = Message::Text(json_message.to_string());

        let peers: tokio::sync::MutexGuard<Vec<mpsc::UnboundedSender<Message>>> = peer_list.lock().await;
        for tx in peers.iter() {
            let _ = tx.send(message.clone());
        }
    }
}

async fn generate_random_numbers(number_tx: mpsc::UnboundedSender<i32>) {
    // delay interval
    let mut interval: tokio::time::Interval = tokio::time::interval(Duration::from_millis(100));

    loop {
        interval.tick().await;
        let number = rand::thread_rng().gen_range(1..=10);
        let _ = number_tx.send(number);
    }
}

async fn broadcast_client_count(peers: &Vec<Tx>) {
    // let count_message = Message::text(format!("Connected clients: {}", peers.len()));
    let json_message = serde_json::json!({"connectedClientsCount": peers.len()});
    let count_message = Message::Text(json_message.to_string());
    for tx in peers.iter() {
        let _ = tx.send(count_message.clone());
    }
}

#[tokio::main]
async fn main() {
    let peer_list: PeerList = Arc::new(Mutex::new(Vec::new()));
    // only this machine can access ws://127.0.0.1:8080
    let listener = TcpListener::bind("127.0.0.1:8080").await.expect("Failed to bind");


    // expose to local network: ws://192.168.1.11:8080 (depends on your network)
    // let listener = TcpListener::bind("192.168.1.11:8080").await.expect("Failed to bind");

    // In HCMUE's C502 network:
    // thread 'main' panicked at learning/src/main.rs:7037:65:
    // Failed to bind: Os { code: 49, kind: AddrNotAvailable, message: "Can't assign requested address" }


    let (number_tx, number_rx) = mpsc::unbounded_channel();
    let peer_list_clone: Arc<Mutex<Vec<mpsc::UnboundedSender<Message>>>> = peer_list.clone();

    tokio::spawn(async move {
        generate_random_numbers(number_tx).await;
    });

    tokio::spawn(async move {
        broadcast_random_numbers(peer_list_clone, number_rx).await;
    });

    while let Ok((stream, addr)) = listener.accept().await {
        let peer_list_clone = peer_list.clone();
        tokio::spawn(async move {
            if let Ok(ws_stream) = accept_async(stream).await {
                handle_connection(peer_list_clone, ws_stream, addr).await;
            }
        });
    }
}
*/




/*
// status: worked
// let's modify this to broadcast random hex color code instead of just 1-10 numbers
// features: broadcast generated hex color codes, clients connection/disconnection event
use tokio::net::TcpListener;
use tokio::sync::{mpsc, Mutex};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use futures_util::StreamExt;
use futures_util::SinkExt;
use std::sync::Arc;
use std::time::Duration;
use rand::Rng;

type Tx = mpsc::UnboundedSender<Message>;
type PeerList = Arc<Mutex<Vec<Tx>>>;

async fn handle_connection(peer_list: PeerList, stream: tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>, peer_addr: std::net::SocketAddr) {
    let (tx, mut rx) = mpsc::unbounded_channel();
    {
        let mut peers: tokio::sync::MutexGuard<Vec<mpsc::UnboundedSender<Message>>> = peer_list.lock().await;
        peers.push(tx.clone());
        broadcast_client_count(&peers).await;
        println!("Client connected: {}", peer_addr);
        println!("Current peers: {}", peers.len());
    }

    let (mut ws_sender, mut ws_receiver) = stream.split();

    let send_task = tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            if ws_sender.send(message).await.is_err() {
                break;
            }
        }
    });

    let receive_task: tokio::task::JoinHandle<()> = tokio::spawn(async move {
        while ws_receiver.next().await.is_some() {}
    });

    tokio::select! {
        _ = send_task => {}
        _ = receive_task => {}
    }

    {
        let mut peers: tokio::sync::MutexGuard<Vec<mpsc::UnboundedSender<Message>>> = peer_list.lock().await;
        if let Some(index) = peers.iter().position(|peer_tx| peer_tx.same_channel(&tx)) {
            peers.remove(index);
            broadcast_client_count(&peers).await;
            println!("Client disconnected: {}", peer_addr);
            println!("Current peers: {}", peers.len());
        }
    }
}

async fn broadcast_random_color_hex(peer_list: PeerList, mut color_rx: mpsc::UnboundedReceiver<String>) {
    while let Some(color_code) = color_rx.recv().await {
        let json_message = serde_json::json!({"color": color_code});
        let message = Message::Text(json_message.to_string());

        let peers: tokio::sync::MutexGuard<Vec<mpsc::UnboundedSender<Message>>> = peer_list.lock().await;
        for tx in peers.iter() {
            let _ = tx.send(message.clone());
        }
    }
}

async fn broadcast_client_count(peers: &Vec<Tx>) {
    // let count_message = Message::text(format!("Connected clients: {}", peers.len()));
    let json_message = serde_json::json!({"connectedClientsCount": peers.len()});
    let count_message = Message::Text(json_message.to_string());
    for tx in peers.iter() {
        let _ = tx.send(count_message.clone());
    }
}

#[tokio::main]
async fn main() {
    let peer_list: PeerList = Arc::new(Mutex::new(Vec::new()));
    // only this machine can access ws://127.0.0.1:8080
    let listener = TcpListener::bind("127.0.0.1:8080").await.expect("Failed to bind");


    // expose to local network: ws://192.168.1.11:8080 (depends on your network)
    // let listener = TcpListener::bind("192.168.1.11:8080").await.expect("Failed to bind");

    // In HCMUE's C502 network:
    // thread 'main' panicked at learning/src/main.rs:7037:65:
    // Failed to bind: Os { code: 49, kind: AddrNotAvailable, message: "Can't assign requested address" }


    let (number_tx, number_rx) = mpsc::unbounded_channel();
    let peer_list_clone: Arc<Mutex<Vec<mpsc::UnboundedSender<Message>>>> = peer_list.clone();

    tokio::spawn(async move {
        generate_random_color(number_tx).await;
    });

    tokio::spawn(async move {
        broadcast_random_color_hex(peer_list_clone, number_rx).await;
    });

    while let Ok((stream, addr)) = listener.accept().await {
        let peer_list_clone = peer_list.clone();
        tokio::spawn(async move {
            if let Ok(ws_stream) = accept_async(stream).await {
                handle_connection(peer_list_clone, ws_stream, addr).await;
            }
        });
    }
}

async fn generate_random_color(number_tx: mpsc::UnboundedSender<String>) {
    let mut interval: tokio::time::Interval = tokio::time::interval(Duration::from_millis(500));

    loop {
        interval.tick().await;
        let mut rng = rand::thread_rng();
        let r: u8 = rng.gen_range(0..=255);
        let g: u8 = rng.gen_range(0..=255);
        let b: u8 = rng.gen_range(0..=255);
        let result = format!("{:02X}{:02X}{:02X}", r, g, b);
        let _ = number_tx.send(result);
    }
}
*/



// attempt to update the Rust server to handle user data and maintain a list of connections with user data.
/*      Example terminal output:
Client connected: 127.0.0.1:54115
Current peers: ["tuanhayho"]
Client connected: 127.0.0.1:54127
Current peers: ["tuanhayho", ""]
Client disconnected: 127.0.0.1:54127
Current peers: ["tuanhayho"]
Client connected: 127.0.0.1:54133
Current peers: ["tuanhayho", "thaihq"]
Client disconnected: 127.0.0.1:54133
Current peers: ["tuanhayho"]
Client disconnected: 127.0.0.1:54115
Current peers: []
*/
use tokio::net::TcpListener;
use tokio::sync::{mpsc, Mutex};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use futures_util::{StreamExt, SinkExt};
use serde_json::Value;
use std::sync::Arc;
use std::time::Duration;
use rand::Rng;

type Tx = mpsc::UnboundedSender<Message>;

#[derive(Clone, Debug)]
struct Peer {
    user_name: String,
    tx: Tx,
}

type PeerList = Arc<Mutex<Vec<Peer>>>;

async fn handle_connection(peer_list: PeerList, stream: tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>, peer_addr: std::net::SocketAddr) {
    let (tx, mut rx) = mpsc::unbounded_channel();

    let (mut ws_sender, mut ws_receiver) = stream.split();

    let mut user_name = String::new();

    // Receive the first message with user data
    if let Some(Ok(Message::Text(user_data))) = ws_receiver.next().await {
        if let Ok(json_data) = serde_json::from_str::<Value>(&user_data) {
            if let Some(name) = json_data["user"]["userName"].as_str() {
                user_name = name.to_string();
            }
        }
    }

    {
        let mut peers = peer_list.lock().await;
        peers.push(Peer { user_name: user_name.clone(), tx: tx.clone() });
        broadcast_user_list(&peers).await;
        println!("Client connected: {}", peer_addr);
        println!("Current peers: {:?}", peers.iter().map(|p| &p.user_name).collect::<Vec<&String>>());
    }

    let send_task = tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            if ws_sender.send(message).await.is_err() {
                break;
            }
        }
    });

    // let receive_task: tokio::task::JoinHandle<()> = tokio::spawn(async move {
    //     while ws_receiver.next().await.is_some() {}
    // });

    let receive_task = tokio::spawn(async move {
        while let Some(msg) = ws_receiver.next().await {
            if let Ok(Message::Close(_)) = msg {
                break;
            }
        }
    });

    tokio::select! {
        _ = send_task => {}
        _ = receive_task => {}
    }

    // Wait for the send task to complete
    // send_task.await.unwrap();

    {
        let mut peers = peer_list.lock().await;
        if let Some(index) = peers.iter().position(|peer| peer.user_name == user_name) {
            peers.remove(index);
            broadcast_user_list(&peers).await;
            println!("Client disconnected: {}", peer_addr);
            println!("Current peers: {:?}", peers.iter().map(|p| &p.user_name).collect::<Vec<&String>>());
        }
    }
}

async fn broadcast_random_color_hex(peer_list: PeerList, mut color_rx: mpsc::UnboundedReceiver<String>) {
    while let Some(color_code) = color_rx.recv().await {
        let json_message = serde_json::json!({"color": color_code});
        let message = Message::Text(json_message.to_string());

        let peers = peer_list.lock().await;
        for peer in peers.iter() {
            let _ = peer.tx.send(message.clone());
        }
    }
}

async fn broadcast_user_list(peers: &Vec<Peer>) {
    let user_names: Vec<&String> = peers.iter().map(|p| &p.user_name).collect();
    let json_message = serde_json::json!({"users": user_names});
    let user_list_message = Message::Text(json_message.to_string());
    for peer in peers.iter() {
        let _ = peer.tx.send(user_list_message.clone());
    }
}

#[tokio::main]
async fn main() {
    let peer_list: PeerList = Arc::new(Mutex::new(Vec::new()));
    let listener = TcpListener::bind("127.0.0.1:8080").await.expect("Failed to bind");

    let (color_tx, color_rx) = mpsc::unbounded_channel();
    let peer_list_clone = peer_list.clone();

    tokio::spawn(async move {
        generate_random_color(color_tx).await;
    });

    tokio::spawn(async move {
        broadcast_random_color_hex(peer_list_clone, color_rx).await;
    });

    while let Ok((stream, addr)) = listener.accept().await {
        let peer_list_clone = peer_list.clone();
        tokio::spawn(async move {
            if let Ok(ws_stream) = accept_async(stream).await {
                handle_connection(peer_list_clone, ws_stream, addr).await;
            }
        });
    }
}

async fn generate_random_color(color_tx: mpsc::UnboundedSender<String>) {
    let mut interval = tokio::time::interval(Duration::from_millis(200));

    loop {
        interval.tick().await;
        let mut rng = rand::thread_rng();
        let r: u8 = rng.gen_range(0..=255);
        let g: u8 = rng.gen_range(0..=255);
        let b: u8 = rng.gen_range(0..=255);
        let color_code = format!("{:02X}{:02X}{:02X}", r, g, b);
        let _ = color_tx.send(color_code);
    }
}
