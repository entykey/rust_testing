use std::time::Instant;

fn sum_array(nums: &[i64]) -> i64 {
    nums.iter().sum()
}

fn main() {
    let nums: Vec<i64> = (1..=1000000).collect();
    let start_time = Instant::now();
    let result = sum_array(&nums);

    println!("Result: {}", result);
    // end of main
    let duration: std::time::Duration = start_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\nExecution time: {:?} ({:?} ms)", duration, elapsed_ms);
}
