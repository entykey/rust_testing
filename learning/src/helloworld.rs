/*  String Split Into 2 Variable In Rust

    https://www.hackertouch.com/rust-programming-language/rust-split-string-into-two-variables.html

*/

use std::time::Instant;

// bonus: return an iterator in Rust
fn generate_numbers() -> impl Iterator<Item = i32> {
    (1..=5).into_iter()
}
fn create_adder(a: u32) -> impl Fn(u32) -> u32 {
    move |b| a + b
}

// enum:
#[allow(dead_code)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}
#[allow(dead_code)]
fn which_way_str(go: Direction) -> &'static str {
    match go {
        Direction::Up => "up",
        Direction::Down => "down",
        Direction::Left => "left",
        Direction::Right => "right",
    }
}
#[allow(dead_code)]
fn which_way_print(go: Direction) {
    match go {
        Direction::Up => println!("up"),
        Direction::Down => println!("down"),
        Direction::Left => println!("left"),
        Direction::Right => println!("right"),
    }
}

fn total_price(prices: &[i32]) -> i32 { // &Vec<i32>
    prices.iter().sum()
}

fn count_vowels(words: &String)-> usize{
    let wovels_count = words.chars().into_iter().filter(|x|(*x=='a') | (*x=='e')| (*x=='i')| (*x=='o')|(*x=='u')).count();
    wovels_count
}

fn add_greeting_target(greeting: &mut String) {
    greeting.push_str("world!");
}

fn get_type<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
fn main() {

    let start = Instant::now();

    // string split
	let my_string = "Hello, world!";
    if let Some((first, second)) = my_string.split_once(",") {
        println!("First part: {}", first); // prints "Hello"
        println!("Second part: {}", second); // prints " world!"
    }


    // count vowels:
    let name="yilmaz".to_string();
    println!("number of vowels: {}",count_vowels(&name));

    // push_str in String
    let mut greeting_string = String::from("Hello ");
    add_greeting_target(&mut greeting_string);
    
    println!("{}", greeting_string);


    // iter sum
    let input_number = vec![1, 2, 3];
    let total = total_price(&input_number);

    println!("{}", total);


    // adding float to integer:
    let int_val: u32 = 15;
    let float_val: f64 = 4.5;
    let adding_int_to_float_result = float_val + f64::from(int_val);

    println!("adding int to float result: {}", adding_int_to_float_result); // 19.5


    // In Rust the type-conversions are explicit, not automatic.
    let bw: f32 = 1./3.;

    let d1: i32 = (4./bw) as i32;
    let d2: i32 = (4 as f32/bw) as i32;
    println!("4/(1/3) --> both are 12: {} {}", d1, d2);
    
    
    
    // return an impl of iterator
    let numbers = generate_numbers();
    println!("datatype of numbers: {}", get_type(&numbers));
    
    for number in numbers {
        //println!("datatype of number: {}", get_type(&number));    // i32
        println!("{}", number);
    }
    
    
    
    // example of return impl of u32
    let adder = create_adder(5);    // type: playground::create_adder::{{closure}}
    let result = adder(22);         // type: u32
    
    println!("Result: {}", result); // prints "Result: 8"
    
    
    
    
    // Anonymous closure
    let anonymous_add = |a, b| a + b;           // type: playground::main::{{closure}}
    let result_anonymous = anonymous_add(2, 3); // type: i32
    println!("Anonymous closure result: {}", result_anonymous);

    // Named closure
    let named_add = |a, b| {                    // type: playground::main::{{closure}}
        let sum = a + b;
        println!("Sum: {}", sum);
        sum
    };
    let result_named = named_add(4, 5);
    println!("Named closure result: {}", result_named);

    let go = Direction::Left;
    println!("which way: {}", which_way_str(go));


    let duration = start.elapsed();
    let elapsed_ms = duration.as_secs_f64() * 1000.0;
    println!("\nExecution time: {:?} ({:?} ms)", duration, elapsed_ms);
}