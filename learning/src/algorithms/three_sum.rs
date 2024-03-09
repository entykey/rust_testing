// Three Sum solution

/*
the 3 sum problem is significantly more complex than the 2 sum problem. 
In the 2 sum problem, we only need to find a pair of elements that add up to the target value, 
and we can achieve this with a simple two-pointer approach in linear time.

However, in the 3 sum problem, we need to find all unique triplets that sum up to the 
target value, and each element can only be used once in each triplet. This requires a 
more advanced approach, often involving nested loops or backtracking, leading to a higher time complexity.

The complexity of the 3 sum problem is typically O(n^2) due to the need to explore 
all possible combinations of three elements in the array. As the size of the input 
array increases, the computation time and resources required also increase, making 
it more challenging to solve efficiently.
*/

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
// winner using while loop with no sum variable
// Runtime: 43 ms - Beats 78.57% of users with Rust
// Memory: 4.08mb - Beats 62.18% of users with Rust
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let n = nums.len();

        if n < 3 {
            return result;
        }

        let mut nums = nums;
        nums.sort();

        for i in 0..n - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue; // Skip duplicates
            }

            let mut left = i + 1;
            let mut right = n - 1;

            while left < right {
                let sum = nums[i] + nums[left] + nums[right];

                if sum == 0 {
                    result.push(vec![nums[i], nums[left], nums[right]]);
                    // Skip duplicates
                    while left < right && nums[left] == nums[left + 1] {
                        left += 1;
                    }
                    while left < right && nums[right] == nums[right - 1] {
                        right -= 1;
                    }
                    left += 1;
                    right -= 1;
                } else if sum < 0 {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }

        result
    }
}

/*
#[allow(dead_code)]
// just another solution
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        nums.sort();

        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut left = i + 1;
            let mut right = nums.len() - 1;

            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                if sum > 0 {
                    right -= 1;
                } else if sum < 0 {
                    left += 1
                } else {
                    res.push(vec![nums[i], nums[left], nums[right]]);
                    left += 1;

                    while nums[left] == nums[left - 1] && left < right {
                        left += 1;
                    }
                }
            }
        }

        return res;
    }
}
*/


fn main() {
    let main_time: std::time::Instant = std::time::Instant::now();

    // Test the three_sum function with the given examples
    let nums1 = vec![-1, 0, 1, 2, -1, -4];
    println!("{:?}", Solution::three_sum(nums1)); // Output: [[-1, -1, 2], [-1, 0, 1]]

    let nums2 = vec![0, 1, 1];
    println!("{:?}", Solution::three_sum(nums2)); // Output: []

    let nums3 = vec![0, 0, 0];
    println!("{:?}", Solution::three_sum(nums3)); // Output: [[0, 0, 0]]

    // end of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\nExecution time: {:?} ({:?} ms)", duration, elapsed_ms);
}
