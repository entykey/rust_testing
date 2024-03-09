// connection pooling stimulation

use std::sync::{mpsc, Arc, Mutex};
use std::thread;

const POOL_SIZE: usize = 5; // The size of the connection pool

// Simulation of a resource (e.g., database connection)
type Connection = i32;

fn create_connection() -> Connection {
    // Simulate creating a new resource/connection
    static mut COUNTER: i32 = 0;
    unsafe {
        COUNTER += 1;
        COUNTER
    }
}

fn main() {
    // Create a shared connection pool using a Mutex and Arc
    let pool: Arc<Mutex<Vec<Connection>>> = Arc::new(Mutex::new(Vec::new()));

    // Create the connection pool
    for _ in 0..POOL_SIZE {
        let connection = create_connection();
        pool.lock().unwrap().push(connection);
    }

    // Function to simulate acquiring and releasing connections
    fn simulate_usage(pool: Arc<Mutex<Vec<Connection>>>, id: usize) {
        // Simulate acquiring a connection
        let connection = pool.lock().unwrap().pop();

        match connection {
            Some(conn) => {
                println!("Thread {} acquired connection: {}", id, conn);

                // Simulate using the connection
                // For demonstration, we'll just sleep for a random duration
                let sleep_duration = std::time::Duration::from_millis(rand::random::<u64>() % 1000);
                thread::sleep(sleep_duration);

                // Simulate releasing the connection
                println!("Thread {} released connection: {}", id, conn);
                pool.lock().unwrap().push(conn);
            }
            None => {
                println!("Thread {} failed to acquire a connection", id);
            }
        }
    }

    // Simulate multiple threads using connections from the pool concurrently
    let (sender, receiver) = mpsc::channel();
    for i in 0..10 {
        let pool_clone = pool.clone();
        let sender_clone = sender.clone();
        thread::spawn(move || {
            simulate_usage(pool_clone, i);
            sender_clone.send(()).unwrap();
        });
    }

    // Wait for all threads to finish
    for _ in 0..10 {
        receiver.recv().unwrap();
    }
}
