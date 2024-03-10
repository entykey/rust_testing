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
// */




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