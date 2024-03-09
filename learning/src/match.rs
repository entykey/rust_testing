// match example
// https://doc.rust-lang.org/rust-by-example/flow_control/match.html

fn main() {
    let start = std::time::Instant::now();


    let number = 18;
    // TODO ^ Try different values for `number`

    println!("Tell me about {}", number);
    match number {
        // Match a single value
        1 => println!("One!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // TODO ^ Try adding 13 to the list of prime values
        // Match an inclusive range
        13..=19 => println!("A teen"),
        // Handle the rest of cases
        _ => println!("Ain't special"),
        // TODO ^ Try commenting out this catch-all arm
    }

    let boolean = true;
    // Match is an expression too
    let binary = match boolean {
        // The arms of a match must cover all the possible values
        false => 0,
        true => 1,
        // TODO ^ Try commenting out one of these arms
    };

    println!("{} -> {}", boolean, binary);


    let duration = start.elapsed();
    let elapsed_ms = duration.as_secs_f64() * 1000.0;
    println!("\nExecution time: {:?} ({:?} ms)", duration, elapsed_ms);
}