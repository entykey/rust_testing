// https://leetcode.com/problems/majority-element/description/
use std::collections::HashMap;

#[allow(dead_code)]
struct Solution;


// bad performance:
#[allow(dead_code)]
impl Solution {
    pub fn majority_element1(nums: Vec<i32>) -> i32 {
        let mut counts = HashMap::new();
        let n = nums.len() as i32;

        for num in nums {
            let count = counts.entry(num).or_insert(0);
            *count += 1;
        }

        for (num, count) in counts {
            if count > n / 2 {
                return num;
            }
        }

        unreachable!(); // Majority element always exists, so this line will never be reached
    }
}

// best performance, using Boyer-Moore Majority Vote algorithm implementation
// Runtime: 3ms - Beats 81.29 %of users with Rust
// Memory: 2.28mb - Beats 98.34 %of users with Rust
#[allow(dead_code)]
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut candidate = 0;
        let mut count = 0;

        for num in nums {
            if count == 0 {
                candidate = num;
                count = 1;
            } else if candidate == num {
                count += 1;
            } else {
                count -= 1;
            }
        }

        candidate
    }
}

fn main() {
    let main_time: std::time::Instant = std::time::Instant::now();


    let vec1 = vec![2,2,1,1,1,2,2];
    let result1 = Solution::majority_element(vec1);
    assert_eq!(result1, 2);
    println!("Output: {}", result1);


    // end of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\nExecution time: {:?} ({:?} ms)", duration, elapsed_ms);
}