// RwLock<> approach (for high i/o reading)

use std::sync::{Arc, RwLock};
use std::thread;

struct CourseRegistration {
    capacity: usize,
    registered_students: usize,
    registration_lock: RwLock<()>,
}

impl CourseRegistration {
    fn new(capacity: usize) -> Self {
        CourseRegistration {
            capacity,
            registered_students: 0,
            registration_lock: RwLock::new(()),
        }
    }

    fn register_student(&mut self) {
        if self.registered_students < self.capacity {
            // Acquire a write lock before modifying the number of registered students
            let _write_lock = self.registration_lock.write().unwrap();
            self.registered_students += 1;
            println!("Registration successful! Slots left: {}", self.capacity - self.registered_students);
        } else {
            println!("Class is full. Registration failed.");
        }
    }
}

fn main() {
    let main_time: std::time::Instant = std::time::Instant::now();


    let registration_system = Arc::new(RwLock::new(CourseRegistration::new(50)));
    let mut handles = vec![];

    for _ in 0..=50 {
        let registration_system_clone = Arc::clone(&registration_system);
        let handle = thread::spawn(move || {
            // Simulate multiple students trying to register at the same time
            registration_system_clone.write().unwrap().register_student();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }


    // end of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\n⌛️ Execution time: {:?} ({:?} ms)", duration, elapsed_ms);
}



// Mutex<> approach (for high speed)
/*
use std::sync::{Arc, Mutex};
use std::thread;

struct CourseRegistration {
    capacity: usize,
    registered_students: usize,
    registration_lock: Mutex<()>,
}

impl CourseRegistration {
    fn new(capacity: usize) -> Self {
        CourseRegistration {
            capacity,
            registered_students: 0,
            registration_lock: Mutex::new(()),
        }
    }

    fn register_student(&mut self) {
        // Acquire the lock before accessing shared data
        let _lock = self.registration_lock.lock().unwrap();
        if self.registered_students < self.capacity {
            self.registered_students += 1;
            println!("Registration successful! Slots left: {}", self.capacity - self.registered_students);
        } else {
            println!("Class is full. Registration failed.");
        }
    }
}

fn main() {
    let main_time: std::time::Instant = std::time::Instant::now();


    let registration_system = Arc::new(Mutex::new(CourseRegistration::new(50)));
    let mut handles = vec![];

    for _ in 0..=50 {
        let registration_system_clone = Arc::clone(&registration_system);
        let handle = thread::spawn(move || {
            // Simulate multiple students trying to register at the same time
            registration_system_clone.lock().unwrap().register_student();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }


    // end of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\n⌛️ Execution time: {:?} ({:?} ms)", duration, elapsed_ms);
}
*/