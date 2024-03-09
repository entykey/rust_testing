#![allow(unused)]
use std::{rc::Rc, sync::Mutex};
use std::time::Instant;



// Function that takes a mutable raw pointer to an integer and modifies the value
fn modify_value_add_ten(ptr: *mut i32) {
    // We need to ensure that the pointer is not null before dereferencing it
    if !ptr.is_null() {
        unsafe {
            *ptr += 10;
        }
    }
}
struct User {
    name: String,
    age: i32,
}

fn main() {
    let main_time: Instant = Instant::now();


    // by using boxed slice, we can avoid unnecessary reallocations and potential overhead caused by resizing vectors or dynamic collections
    // loop through a boxed slice using iter()
    let data: Box<[i32]> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 9735].into_boxed_slice();

    for item in data.iter() {
        println!("{}", item);
    }

    // Use map to transform the values
    let transformed_data_as_vector: Box<[i32]> = data
        .iter()
        .map(|&x| x * 2)
        .collect::<Vec<_>>()  // Collect back to a vector, but you can collect to a boxed slice as well
        .into_boxed_slice();

    // Use map to transform the values and collect strait into a boxed slice
    let transformed_data_as_boxed_slice: Box<[i32]> = data
        .iter()
        .map(|&x| x * 2)
        .collect(); // Collect strait into boxed slice

    // Print the transformed data
    println!("Original data: {:?}", data);
    println!("Transformed data: {:?}", transformed_data_as_vector);
    println!("Transformed data: {:?}", transformed_data_as_boxed_slice);


    println!();

    // new instance of the RC smart pointer
    let x = Rc::new(5);
    let y = Rc::clone(&x);

    println!("x = {}, y = {}", x, y);

    // new instance of the mutex smart pointer
    let counter = Mutex::new(0);

    {
        let mut num = counter.lock().unwrap();
        *num += 1;
    }

    println!("Result: {}", *counter.lock().unwrap());

    // map example
    let numbers = vec![1, 2, 3, 4, 5];
    println!("Original numbers: {:?}", numbers);

    // Multiply each number by 2 using map
    // Use collect to convert the iterator back to a vector
    let multiplied: Vec<i32> = numbers.iter().map(|&x| x * 2).collect(); // The `|...|` syntax in closures is used to define the parameters and the body of the closure

    println!("After multiplied by 2: {:?}", multiplied); // Output: [2, 4, 6, 8, 10]

    // Add two numbers together using a closure (multiple parameter in parameter list: `|...|`)
    let sum = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("Sum: {}", sum);
    


    println!();
    // Actix Diesel example: Assuming the `query_result` is a Vec<User> obtained from the database query
    let mut query_result: Vec<User> = vec![     // mutable so that we can sort
        User {
            name: "Alex".to_string(),
            age: 12,
        },
        User {
            name: "John".to_string(),
            age: 35,
        },
        User {
            name: "Nguyễn Hữu Anh Tuấn".to_string(),
            age: 19,
        },
        User {
            name: "Bob".to_string(),
            age: 45,
        },
    ];

    // sorting:
    query_result.sort_unstable_by(|a, b| b.age.cmp(&a.age));


    let transformed_result: Vec<String> = query_result
        .iter()
        .filter(|user| user.age > 10)
        .map(|user| format!("Name: {}, Age: {}", user.name, user.age))
        .collect();

    let transformed_result_as_boxed_slice: Box<[String]> = query_result
        .iter()
        .filter(|user| user.age > 10)
        .map(|user| format!("Name: {}, Age: {}", user.name, user.age))
        .collect();

    for result in transformed_result {
        println!("{}", result);
    }
    println!();
    for result in transformed_result_as_boxed_slice.iter() {
        println!("{}", result);
    }




    println!();
    fn divide(numerator: i32, denominator: i32) -> Option<i32> {
        if denominator == 0 {
            return None;
        } else {
            return Some(numerator / denominator);
        }
    }
    match divide(103, 2) {
        Some(solution) => println!("The answer is {}", solution),
        None => println!("Your numerator was about to be divided by zero :)")
    }




    // raw pointer example (use for actix database connection estasblishing pointer)
    let mut value = 42;

    // Get a raw pointer to the `value`
    let ptr_value: *mut i32 = &mut value;

    // Call the function to modify the value through the raw pointer
    modify_value_add_ten(ptr_value);

    // After calling `modify_value`, the `value` will be modified
    println!("Modified value: {}", value);



    // end of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\nExecution time: {:?} ({:?} ms)", duration, elapsed_ms);
}
