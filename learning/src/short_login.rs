#![allow(unused)]
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::Error;
use std::mem::{size_of, size_of_val};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;


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

fn main() {
    let main_time: Instant = Instant::now();
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

    println!("Threads finished: {}", is_finished(&roles_mutex, &departments_mutex));

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


    // end of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\nExecution time: {:?} ({:?} ms)", duration, elapsed_ms);
}