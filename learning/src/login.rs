// What is `.lock()` ?
// When you call lock() on a Mutex, it returns a smart pointer called MutexGuard. The MutexGuard is
// responsible for ensuring that only one thread at a time can access the shared data protected by the Mutex.
// While the MutexGuard is in scope, it guarantees exclusive access to the data, and when it goes out
// of scope (e.g., at the end of the closure or function), the lock is released, allowing other threads to access the data.

// How it works:

// 1. When a thread wants to access shared data protected by a Mutex, it calls lock() on the Mutex.

// 2. If the Mutex is not currently locked by another thread, lock() will return immediately, 
// and the thread gains exclusive access to the data.

// 3. If the Mutex is locked by another thread, the calling thread will be blocked (i.e., it will wait)
// until the lock is released by the other thread. This ensures that only one thread can access the shared data at any given time.

// 4. Once the thread has access to the data (via the returned MutexGuard), it can read or modify the data safely.

// 5. When the MutexGuard goes out of scope (at the end of the closure or function), 
// the lock is automatically released, allowing other threads to acquire the lock and access the shared data.


#![allow(unused)]
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::Error;
use std::mem::{size_of, size_of_val};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::time::Instant;

// colored print:
use termcolor::{StandardStream, Color, ColorChoice, ColorSpec, WriteColor};
use std::io::Write;


#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone)]
struct Department {
    ID: i32,
    Name: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone)]
struct UserData {
    Id: String,
    // Roles: Vec<String>,
    // Departments: Vec<Department>,
    Roles: Box<[String]>,
    Departments: Box<[Department]>,
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
struct Product {
    ID: i32,
    Name: String,
    Price: f64,
}

fn get_all_role_names_of_a_user_by_user_id(_user_id: &str) -> Box<[String]> {   // Vec<String>
    // fake method so `user_id` s unused -> `_user_id`
    // Simulate delay
    //thread::sleep(Duration::from_millis(60));

    // Fake result
    vec![
        "Admin".to_string(),
        "Member".to_string(),
        "Moderator".to_string(),
    ]
    .into_boxed_slice()
}

fn get_departments_of_a_user(_user_id: &str) -> Box<[Department]> {
    // fake method so `user_id` s unused -> `_user_id`
    // Simulate delay
    //thread::sleep(Duration::from_millis(120));

    // Fake result
    vec![
        Department {
            ID: 1,
            Name: "IT".to_string(),
        },
        Department {
            ID: 2,
            Name: "Sales".to_string(),
        },
        Department {
            ID: 3,
            Name: "Marketing".to_string(),
        },
    ]
    .into_boxed_slice()
}

fn is_finished(roles_mutex: &Mutex<Box<[String]>>, departments_mutex: &Mutex<Box<[Department]>>) -> bool {
    let roles_guard: MutexGuard<'_, Box<[String]>> = roles_mutex.lock().unwrap();
    let departments_guard: MutexGuard<'_, Box<[Department]>> = departments_mutex.lock().unwrap();
    !roles_guard.is_empty() && !departments_guard.is_empty()
}

#[derive(Serialize, Deserialize, Clone)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

#[allow(dead_code)]
fn merge_string_and_reverse(a: u32, b: &str) -> String {
    format!("💖 {b} {a}")
}

// serde_json testing
#[allow(dead_code)]
fn typed_example() -> Result<(), Error> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"{
                    "name": "John Doe",
                    "age": 43,
                    "phones": [
                      "+44 1234567",
                      "+44 2345678"
                    ]
                  }"#;

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let p: Person = serde_json::from_str(data)?;

    // Do things just like with any other Rust data structure.
    println!("Please call {} at the number {}", p.name, p.phones[0]);

    Ok(())
}
#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
struct Address {
    street: String,
    city: String,
}
#[allow(dead_code)]
fn print_an_address() -> Result<(), Error> {
    // Some data structure.
    let address = Address {
        street: "10 Downing Street".to_owned(),
        city: "London".to_owned(),
    };

    // Serialize it to a JSON string.
    let j = serde_json::to_string(&address)?;

    // Print, write to a file, or send to an HTTP server.
    println!("{}", j);

    Ok(())
}

fn main() {
    let main_time: Instant = Instant::now();

    // colored print:
    // Create a StandardStream for writing colored output
    let mut stdout = StandardStream::stdout(ColorChoice::Always);


    let user_id = Arc::new("123".to_string());  // to share ownership of user_id between the threads

    // let roles_mutex = Arc::new(Mutex::new(Vec::new()));
    // let departments_mutex = Arc::new(Mutex::new(Vec::new()));
    let roles_mutex = Arc::new(Mutex::new(Box::new([]) as Box<[String]>));
    let departments_mutex = Arc::new(Mutex::new(Box::new([]) as Box<[Department]>));

    let roles_mutex_clone = roles_mutex.clone();
    let departments_mutex_clone = departments_mutex.clone();



    // Each thread needs its own copy of the user_id to avoid data races & conflict and ensure thread safety !
    // creates a clone (a deep copy) of the original user_id string
    //let user_id_clone: String = user_id.clone();

    // OPTIMIZED: creates a new atomic reference-counted pointer (Arc) (It increments the reference count of the Arc so that multiple threads can share ownership of the data)
    let user_id_clone: Arc<String> = Arc::clone(&user_id);

    // the thread::JoinHandle<()> type represents a handle to a spawned thread
    let handle1: thread::JoinHandle<()> = thread::spawn(move || {

        let thread_id: thread::ThreadId = thread::current().id();
        println!("Thread 1 ID: {:?}", thread_id);

        let start: Instant = Instant::now();
        let roles = get_all_role_names_of_a_user_by_user_id(&user_id_clone);    // &user_id_clone
        let duration: std::time::Duration = start.elapsed();
        println!(
            "Roles obtained in {:.6}ms",
            duration.as_micros() as f64 / 1000.0
        );

        let mut roles_guard = roles_mutex_clone.lock().unwrap();
        *roles_guard = roles;
    });



    // Each thread needs its own copy of the user_id to avoid data races & conflict and ensure thread safety !
    // creates a clone (a deep copy) of the original user_id string
    //let user_id_clone: String = user_id.clone();

    // OPTIMIZED: creates a new atomic reference-counted pointer (Arc) (It increments the reference count of the Arc so that multiple threads can share ownership of the data)
    let user_id_clone: Arc<String> = Arc::clone(&user_id);

    // the thread::JoinHandle<()> type represents a handle to a spawned thread
    let handle2: thread::JoinHandle<()> = thread::spawn(move || {

        let thread_id: thread::ThreadId = thread::current().id();
        println!("Thread 2 ID: {:?}", thread_id);

        let start: Instant = Instant::now();
        let departments = get_departments_of_a_user(&user_id_clone);
        let duration: std::time::Duration = start.elapsed();
        println!(
            "Departments obtained in {:.6}ms",
            duration.as_secs_f64() * 1000.0
        );

        let mut departments_guard = departments_mutex_clone.lock().unwrap();
        *departments_guard = departments;
    });

    // make the main thread waits for the two spawned threads to
    // complete their execution using thread::JoinHandle<()>.join().unwrap()
    while !is_finished(&roles_mutex, &departments_mutex) {
        thread::yield_now();
    }

    // Set the color specification for the text
    let mut color_spec = ColorSpec::new();
    color_spec.set_fg(Some(Color::Cyan));

    // Write the colored text to stdout
    stdout.set_color(&color_spec).unwrap();

    println!("♫ Threads finished: {}", is_finished(&roles_mutex, &departments_mutex));

    // Reset the text color back to the default
    stdout.reset().unwrap();

    handle1.join().unwrap();
    handle2.join().unwrap();

    // vector approach
    // let handles = vec![handle1, handle2];

    // handles.into_iter().for_each(|handle| {
    //     handle.join().unwrap();
    // });

    let roles_guard = roles_mutex.lock().unwrap();
    let departments_guard = departments_mutex.lock().unwrap();

    let user_data = UserData {
        Id: user_id.clone().to_string(),
        // Roles: roles_guard.clone(),
        // Departments: departments_guard.clone(),
        Roles: roles_guard.clone(),
        Departments: departments_guard.clone(),
    };

    let json_data: String = serde_json::to_string(&user_data).unwrap();
    println!("{}", json_data);



    let products = vec![
        Product {
            ID: 1,
            Name: "Product A".to_string(),
            Price: 10.99,
        },
        Product {
            ID: 2,
            Name: "Product B".to_string(),
            Price: 20.99,
        },
        Product {
            ID: 3,
            Name: "Product C".to_string(),
            Price: 15.99,
        },
        Product {
            ID: 4,
            Name: "Product D".to_string(),
            Price: 12.99,
        },
        Product {
            ID: 5,
            Name: "Product E".to_string(),
            Price: 19.99,
        },
    ];

    println!("");

    // Example 1: Filtering (get the products with price > 15.0)
    let filtered_products: Vec<String> = products
        .iter()
        .filter(|product| product.Price > 15.0)
        .map(|product| product.Name.clone())
        .collect();

    println!("Filtered products:");
    for item in filtered_products {
        println!("{}", item);
    }

    // Example 2: Aggregation (Sum)
    let total_price: f64 = products.iter().map(|product| product.Price).sum();

    println!("Total price: {:.2}", total_price);

    println!("\nserde_json example:");
    if let Err(err) = typed_example() {
        eprintln!("Error: {}", err);
    }
    if let Err(err) = print_an_address() {
        eprintln!("Error: {}", err);
    }

    // Arc, Mutex example:
    //(Arc is safe to use in concurent situations), if you’re just performing
    // operations on values within a single thread, your code can run faster
    // if it doesn’t have to enforce the guarantees atomics provide.
    let counter: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    let mut handles: Vec<thread::JoinHandle<()>> = vec![];

    for _ in 0..10 {
        let counter: Arc<Mutex<i32>> = Arc::clone(&counter);
        let handle: thread::JoinHandle<()> = thread::spawn(move || {
            let mut num: MutexGuard<'_, i32> = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

    println!("Merged string: {}", merge_string_and_reverse(927, "label")); // => "label 927"


    // use boxed slice for better performance due to smaller type size (https://nnethercote.github.io/perf-book/type-sizes.html)
    let v: Vec<u32> = vec![1, 2, 3];
    let expected_v = 3 * size_of::<usize>();
    assert_eq!(size_of_val(&v), expected_v);    // if matches, continue the program
    println!("size_of_val(&v) = {}, expected: {}", size_of_val(&v), expected_v);

    let bs: Box<[u32]> = v.into_boxed_slice();
    let expected_bs = 2 * size_of::<usize>();
    assert_eq!(size_of_val(&bs), expected_bs);  // if matches, continue the program
    println!("size_of_val(&bs) = {}, expected: {}", size_of_val(&bs), expected_bs);


    
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\nExecution time: {:?} ({:?} ms)", duration, elapsed_ms);
}
