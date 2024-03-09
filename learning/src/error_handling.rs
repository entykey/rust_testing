#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use std::env;

// fn divide(a: f64, b: f64) -> Result<f64, String> {
//     if b == 0.0 {
//         Err("Error: Division by zero!".to_string())
//     } else {
//         Ok(a / b)
//     }
// }

// generic divide
#[allow(dead_code)]
fn divide_generic_1<T, U>(a: T, b: U) -> Result<T, String>
where
    T: std::ops::Div<Output = T> + std::cmp::PartialEq + From<i32>, // Add the necessary trait bounds
    U: Into<T> + std::cmp::PartialEq + From<i32>, // Add Into trait to convert U to T
    //  T must implement std::ops::Div<Output = T> (the division operator),
    // std::cmp::PartialEq (for equality comparison with zero),
    // and From<i32> (to convert the integer literal 0 to the generic type T).
{
    let zero: U = 0.into();
    if b == zero {
        Err("Error: Division by zero!".to_string())
    } else {
        Ok(a / b.into())
    }
}


// using num_traits crate
#[allow(dead_code)]
fn divide_generic_2<T>(a: T, b: T) -> Result<T, String>
where
    T: std::ops::Div<Output = T> + std::cmp::PartialEq + From<i32> + num_traits::Zero, // Add the necessary trait bounds
    //  T must implement std::ops::Div<Output = T> (the division operator),
    // std::cmp::PartialEq (for equality comparison with zero),
    // and From<i32> (to convert the integer literal 0 to the generic type T).
{
    if b ==  T::zero() {
        Err("Error: Division by zero!".to_string())
    } else {
        Ok(a / b)
    }
}

fn double_number(number_str: &str) -> Result<i32, std::num::ParseIntError> {
    number_str.parse::<i32>().map(|n| 2 * n)
}

#[allow(dead_code)]
fn double_number_unwrap(number_str: &str) -> i32 {
    2 * number_str.parse::<i32>().unwrap()
}

fn double_arg(mut argv: env::Args) -> Result<i32, String> {
    argv.nth(1)
        .ok_or("Please give at least one argument".to_owned())
        .and_then(|arg| arg.parse::<i32>().map_err(|err| err.to_string()))
        .map(|n| 2 * n)
}




// main use operator "?", instead of match ... {...}
fn main() -> Result<(), String> {
    let start = std::time::Instant::now();


    let result = divide_generic_1(8.8, 2);

    // Using expect to provide a custom error message
    println!("\n##      expect example:         ##\n");

    // if divide by 0 -> thread 'main' panicked at 'Failed to perform division: "Error: Division by zero!"'
    let value = result.expect("Failed to perform division");
    println!("Result: {}", value);



    // handling panick:
    // Using unwrap_or to provide a default value on error
    let error_result = divide_generic_1(5.0, 0.0);
    //let error_value1 = error_result.unwrap_or(-1.0); // Default value if there's an error
    //println!("Error Result: {}", error_value1);


    // Using expect_err to provide a custom error message for failure
    println!("\n##      expect_err example:         ##\n");

    let error_value2 = error_result.expect_err("Something went wrong!");
    println!("{}", error_value2);

    println!();

    
    // using match instead of unwrap
    println!("\n##      match example:         ##\n");

    match double_number("10") {
        //Ok(n) => assert_eq!(n, 20),
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {:?}", err),
    }
    match double_number("ten") {
        //Ok(n) => assert_eq!(n, 20),
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {:?}", err),
    }


    // test unwrap_or_else
    println!("\n##      unwrap_or_else example:         ##\n");
    let result1 = divide_generic_1(10, 2).unwrap_or_else(|err| {
        println!("Error: {}", err);
        0
    }); // Returns 5
    println!("result1: {}", result1);

    let result2 = divide_generic_1(10, 0).unwrap_or_else(|err| {
        println!("Error: {}", err);
        0
    }); // Prints "Error: Division by zero" and returns 0
    println!("result2: {}", result2);



    println!();
    // arg:
    /*
    let mut argv = env::args();
    let arg: String = argv.nth(1).unwrap(); // error 1
    let n: i32 = arg.parse().unwrap(); // error 2
    println!("{}", 2 * n);
    */
    
    // better handling using match:
    match double_arg(env::args()) {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {}", err), // output: Error: Please give at least one argument
    }

    
    println!();
    // unwrap:  (bad handling, panicked)
    // let n: i32 = double_number_unwrap("10*");
    // assert_eq!(n, 20);
    


    let duration = start.elapsed();
    let elapsed_ms = duration.as_secs_f64() * 1000.0;
    println!("\nExecution time: {:?} ({:?} ms)", duration, elapsed_ms);

    Ok(())
}
