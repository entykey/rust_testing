// https://doc.rust-lang.org/std/borrow/enum.Cow.html

#![allow(unused)]
use std::borrow::Cow;

fn abs_all(input: &mut Cow<'_, [i32]>) {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // Clones into a vector if not already owned.
            input.to_mut()[i] = -v;
        }
    }
}

fn main() {
    let main_time: std::time::Instant = std::time::Instant::now();


    // No clone occurs because `input` doesn't need to be mutated.
    let slice = [0, 1, 2];
    let mut input = Cow::from(&slice[..]);
    abs_all(&mut input);
    println!("input: {:?}", input);

    // Clone occurs because `input` needs to be mutated.
    let slice = [-1, 0, 1];
    let mut input = Cow::from(&slice[..]);
    abs_all(&mut input);
    println!("input: {:?}", input);

    // No clone occurs because `input` is already owned.
    let mut input = Cow::from(vec![-1, 0, 1]);
    abs_all(&mut input);
    println!("input: {:?}", input);



    // end of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\nExecution time: {:?} ({:?} ms)", duration, elapsed_ms);
}