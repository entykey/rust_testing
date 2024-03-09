// https://leetcode.com/problems/remove-duplicates-from-sorted-array/description/
#[allow(dead_code)]
struct Solution;


// 26ms
#[allow(dead_code)]
impl Solution {
    pub fn remove_duplicates1(nums: &mut Vec<i32>) -> i32 {
        let mut i = 0;

        while i < nums.len() - 1 {
            if nums[i] == nums[i + 1] {
                nums.drain(i..i + 1);
            } else {
                i += 1;
            }
        }

        // duplication count:
        nums.len() as i32
    }
}

// 0ms, 2 lines beats all solution:
#[allow(dead_code)]
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        nums.dedup();

        // duplication count:
        nums.len() as i32
    }
}

fn main() {
    let main_time: std::time::Instant = std::time::Instant::now();


    let mut nums = vec![0,0,1,1,1,2,2,3,3,4];
    let expected_nums = vec![0,1,2,3,4];

    let k = Solution::remove_duplicates(&mut nums);

    assert_eq!(nums, expected_nums);
    assert_eq!(k, 5);

    println!("k: {}", k);   // k is duplication count

    // end of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\nExecution time: {:?} ({:?} ms)", duration, elapsed_ms);
}
