
#[allow(dead_code)]
fn reverse_bits_loop(n: u32) -> u32 {
    let mut result = 0;
    let mut num = n;

    for _ in 0..32 {
        result <<= 1;
        if num & 1 == 1 {
            result |= 1;
        }
        num >>= 1;
    }

    result
}

#[allow(dead_code)]
fn reverse_bits_iterator(n: u32) -> u32 {
    // Create an iterator over each bit of the input number
    let bits = (0..32).map(|i| (n >> i) & 1);

    // Fold the bits into a single reversed integer
    let reversed = bits.fold(0, |acc, bit| (acc << 1) | bit);

    reversed
}

fn main() {
    let main_time: std::time::Instant = std::time::Instant::now();


    let n = 43261596;
    let reversed = reverse_bits_iterator(n);
    println!("Original bits: {}", n);
    println!("Reversed bits: {}", reversed);    // 964176192

    // end of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\nExecution time: {:?} ({:?} ms)", duration, elapsed_ms);
}