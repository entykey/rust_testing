// use std::collections::HashSet;
// use std::hash::Hash;

// find duplocated value
fn find_duplicate(nums: &[i32]) -> Option<i32> {
    let mut seen = std::collections::HashSet::new();

    for &num in nums {
        if !seen.insert(num) {
            return Some(num);
        }
    }

    None
}

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + std::hash::Hash,
{
    let mut uniq = std::collections::HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

fn is_sorted<I>(data: I) -> bool
where
    I: IntoIterator,
    I::Item: Ord,
{
    let mut it = data.into_iter();
    match it.next() {
        None => true,
        Some(first) => it.scan(first, |state, next| {
            let cmp = *state <= next;
            *state = next;
            Some(cmp)
        }).all(|b| b),
    }
}



fn main() {
    let main_time: std::time::Instant = std::time::Instant::now();


    // -> cause 20µs of performance overhead
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 12, 13].into_boxed_slice();
    if let Some(duplicate) = find_duplicate(&nums) {
        println!("Duplicate integer: {}", duplicate);
    } else {
        println!("No duplicate found.");
    }


    // test has_unique_elements():
    assert!(!has_unique_elements(vec![10, 20, 30, 10, 50]));
    assert!(has_unique_elements(vec![10, 20, 30, 40, 50]));
    assert!(has_unique_elements(Vec::<u8>::new()));


    // test is_sorted():
    // let unsorted_vec = vec![2, 3, 4];
    // let is_it_sorted = is_sorted(unsorted_vec);
    // println!("is it sorted ? :{}", is_it_sorted);
    assert_eq!(is_sorted(vec![2, 3, 4]), true);

    // end of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\nExecution time: {:?} ({:?} ms)", duration, elapsed_ms);
}