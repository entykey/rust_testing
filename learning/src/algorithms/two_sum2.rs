// Two Sum II - Input Array Is Sorted solution 
// (https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/)

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
// winner using while loop with no sum variable
// Runtime: - ms - Beats 100.00% of users with Rust
// Memory: 2.09mb - Beats 71.73% of users with Rust
// WARN: This solution assumes that the input vector numbers is sorted
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut first: usize = 0;
        let mut second: usize = numbers.len() - 1;

        while first < second {
            if numbers[first] + numbers[second] == target {
                // in required ouput, the first element start with index as 1, not 0, so we added x, y by 1
                return vec![(first as i32 + 1), (second as i32 + 1)];
            } else if numbers[first] + numbers[second] < target {
                first += 1;
            } else {
                second -= 1;
            }
        }
        // empty vec if result is smaller than sum of any pair
        vec![]
    }
}

/*
#[allow(dead_code)]
// use `std::cmp::Ordering` solution
// Runtime: 2ms - Beats 81.07% of users with Rust
// Memory: 2.09mb - Beats 98.40% of users with Rust
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i = 0;
        let mut j = numbers.len() - 1;

        while i < j {
            match (numbers[i] + numbers[j]).cmp(&target) {
                Ordering::Less => i += 1,
                Ordering::Greater => j -= 1,
                Ordering::Equal => return vec![(i + 1) as i32, (j + 1) as i32],
            }
        }
        //unreachable!();
        vec![]
    }
}
*/

/* runtime 3ms: beating 42% rust users (no lib imported)
fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut left = 0;
    let mut right = numbers.len() - 1;

    while left < right {
        let sum = numbers[left] + numbers[right];

        if sum == target {
            return vec![left as i32 + 1, right as i32 + 1];
        } else if sum < target {
            left += 1;
        } else {
            right -= 1;
        }
    }

    vec![]
}
*/

fn main() {
    let main_time: std::time::Instant = std::time::Instant::now();


    // Test the two_sum function with the given examples
    let numbers1 = vec![2, 7, 11, 15];
    let target1 = 9;
    println!("{:?}", Solution::two_sum(numbers1, target1)); // Output: [1, 2]

    let numbers2 = vec![2, 3, 4];
    let target2 = 6;
    println!("{:?}", Solution::two_sum(numbers2, target2)); // Output: [1, 3]

    let numbers3 = vec![-1, 0];
    let target3 = -1;
    println!("{:?}", Solution::two_sum(numbers3, target3)); // Output: [1, 2]

    let numbers4 = vec![2, 3, 5];
    let target4 = 3;
    println!("{:?}", Solution::two_sum(numbers4, target4)); // Output: []

    // end of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\nExecution time: {:?} ({:?} ms)", duration, elapsed_ms);
}