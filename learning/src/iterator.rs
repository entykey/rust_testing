// iterator in Rust
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use core::convert::TryFrom;

// generic iterator
pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T>
    where T: Eq + Copy + std::ops::Rem<Output=T> + TryFrom<i32>
{
    let zero = T::try_from(0).ok().unwrap();
    let two = T::try_from(2).ok().unwrap();
    iter.filter(move |x| *x % two != zero)
}


fn main() {
    let start = std::time::Instant::now();

    let v1 = vec![1, 2, 3, 4, 5];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    println!("total: {}", total);

    // increase all elm by 1
    let increased_v1: Vec<_> = v1.iter().map(|x| x + 1).collect();

    println!("after increased by 1:");
    for val in increased_v1.iter() {
        println!("{}", val);
    }

    // println!("v1 still accessable here:");
    // for val in v1.iter() {
    //     println!("{}", val);
    // }

    // generic iterator
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let odd_numbers: Vec<_> = evens(numbers.into_iter()).collect();
    
    println!("Odd numbers: {:?}", odd_numbers);

    let duration = start.elapsed();
    let elapsed_ms = duration.as_secs_f64() * 1000.0;
    println!("\nExecution time: {:?} ({:?} ms)", duration, elapsed_ms);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3, 4, 5];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 15);

}

#[test]
fn iterator_map() {
    let v1 = vec![1, 2, 3, 4, 5];

    // increase all elm by 1
    let increased_v1: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(increased_v1, vec![2, 3, 4, 5, 6]);
}
