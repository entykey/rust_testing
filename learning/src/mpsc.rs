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