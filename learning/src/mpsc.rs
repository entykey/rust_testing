/*
use std::sync::mpsc;
use std::thread;

fn main() {
    // Create a channel with a capacity of 5 messages
    let (sender, receiver) = mpsc::channel();

    // Create a thread that sends messages to the channel
    let sender_thread = thread::spawn(move || {
        for i in 1..=5 {
            // The thread::current().id() function returns a ThreadId type, 
            // which doesn't directly implement the Display trait needed for printing. We can convert it to a string `{:?} before printing. 
            println!("Thread {:?} sending message {}", thread::current().id(), i);
            sender.send(i).unwrap();
            thread::sleep(std::time::Duration::from_millis(300));
        }
    });

    // Main thread receives messages from the channel
    for message in receiver {
        println!("Thread {:?} received message: {}", thread::current().id(), message);
    }

    // Wait for the sender thread to finish
    sender_thread.join().unwrap();
}
*/
/*
Output:
Thread ThreadId(2) sending message 1
Thread ThreadId(1) received message: 1
Thread ThreadId(2) sending message 2
Thread ThreadId(1) received message: 2
Thread ThreadId(2) sending message 3
Thread ThreadId(1) received message: 3
Thread ThreadId(2) sending message 4
Thread ThreadId(1) received message: 4
Thread ThreadId(2) sending message 5
Thread ThreadId(1) received message: 5
*/







//*
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
//*/
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