#[allow(dead_code)]
fn main() {

    let main_time: std::time::Instant = std::time::Instant::now();


    let mut a = 1; // binary: 00000001
    let b = 3; // binary: 00000011
    let c = 64;    // 010000000

    a |= b; // bitwise OR operation between two values and assigns the result to the left operand.

    println!("a = {}", a); // Output: a = 3 (binary: 00000011)

    a |= c;
    println!("a = {}", a); // Output: a = 67 (binary: 01000011) -> 64 + 2 + 1 = 3 + 64 = 67

    a |= 32;
    println!("a = {}", a); // Output: a = 99 (binary: 01100011) -> 64 + 32 + 2 + 1 = 67 + 32 = 99

    a &= 8;
    println!("a = {}", a); // Output: a = 0 (binary: 00000000)



    let digit = 4;  // (In binary: 00000100):
    println!("Original digit: {}", digit);
    // shift operator `<<` perform bitwise left shift operations on integers
    // where x is the integer value, and n is the number of positions to shift the bits to the left
    let shift_left_1 = 1 << digit;  // Shift one position to the left
    println!("After shifted left: {}", shift_left_1);   // -> 16
    // explaination:
    /*

    Binary representation of 1:         0 0 0 0 0 0 0 1

    Shifted left by 4 positions:        0 0 0 1 0 0 0 0  (which is 16 in decimal)

    */
    let digit2: u64 = 32; // Binary representation: 100000
    let shifted_right_1 = digit2 >> 1; // Shift one position to the right
    let shifted_right_2 = digit2 >> 2; // Shift two positions to the right
    //let shifted_left_1 = 1 << digit2;
    let shifted_left_2 = 1u64 << digit2; // Use u32 to avoid overflow
    println!("Original digit2: {}", digit2);
    println!("Shifted right by 1: {}", shifted_right_1);    // -> 16
    println!("Shifted right by 2: {}", shifted_right_2);    // -> 8
    //println!("Shifted left by 1: {}", shifted_left_1);      // -> (thread 'main' panicked at 'attempt to shift left with overflow',)
    println!("Shifted right by 1u32: {}", shifted_left_2); // Output: 128


    // end of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\nExecution time: {:?} ({:?} ms)", duration, elapsed_ms);

}
