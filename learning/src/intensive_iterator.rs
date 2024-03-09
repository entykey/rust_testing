use std::time::Instant;

fn main() {
    let start = std::time::Instant::now();

    let mut sum: i64 = 0;
    for i in 1..=1_000_000 {
        sum += i;
    }

    let duration = start.elapsed();
    println!("Rust Result: {}", sum);
    println!("Rust Execution time: {} microseconds.", duration.as_micros());
}
