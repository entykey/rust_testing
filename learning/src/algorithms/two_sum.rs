// https://leetcode.com/problems/two-sum

// Runtime: -ms - Beats 100.00%of users with Rust
// Memory: 2.18mb - Beats 78.75%of users with Rust

// WARN: this solution asume that the input vector nums is unsorted, so we need to consider an 
// approach that works for unsorted input as well. One way to do this is by using a HashMap to store 
// the elements of the vector along with their indices as we iterate through the vector. 
// This way, we can quickly check if the complement of the current element (i.e., the value needed to achieve the target) is present in the HashMap.
use std::collections::HashMap;

#[allow(dead_code)]
struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        
        // Create a hash map to store the difference between target and each number in nums
        let mut num_map: HashMap<i32, usize> = HashMap::with_capacity(nums.len());  // = HashMap::new()

        for (index, num) in nums.iter().enumerate() {
            let complement = target - num;

            // Check if such a difference exists in the hash map
            if let Some(&complement_index) = num_map.get(&complement) {
                return vec![complement_index as i32, index as i32];
            }

            // If it doesn't, add the difference between target and the current number to the hash map
            num_map.insert(*num, index);
        }

        vec![]
    }
}

fn main() {
    let main_time: std::time::Instant = std::time::Instant::now();

    // Test the two_sum function with the given examples
    let nums1 = vec![2,7,11,15];
    let target1: i32 = 9;
    println!("{:?}", Solution::two_sum(nums1, target1)); // Output: [0,1]

    let nums2 = vec![3,2,4];
    let target2: i32 = 6;
    println!("{:?}", Solution::two_sum(nums2, target2)); // Output: [1,2]

    let nums3 = vec![0,-1];
    let target3: i32 = -1;
    println!("{:?}", Solution::two_sum(nums3, target3)); // Output: [0, 1]

    // end of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\nExecution time: {:?} ({:?} ms)", duration, elapsed_ms);
}
