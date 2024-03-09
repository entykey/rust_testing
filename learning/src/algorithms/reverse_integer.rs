// https://leetcode.com/problems/reverse-integer/description/
#[allow(dead_code)]
struct Solution;


// Runtime: -ms - Beats 100.00% of users with Rust
// Memory: 2.02mb - Beats 52.99% of users with Rust
#[allow(dead_code)]
impl Solution {
    pub fn reverse1(x: i32) -> i32 {
        let mut num = x;
        let mut result: i32 = 0;

        while num != 0 {
            // Extract the last digit
            let digit = num % 10;

            // Check for potential integer overflow before adding the digit to the result
            if result > i32::MAX / 10 || (result == i32::MAX / 10 && digit > 7) {
                return 0;
            }
            if result < i32::MIN / 10 || (result == i32::MIN / 10 && digit < -8) {
                return 0;
            }

            result = result * 10 + digit;
            num /= 10;
        }

        result
    }
}

// Runtime: -ms - Beats 100.00% of users with Rust
// Memory: 1.99mb - Beats 92.02% of users with Rust
#[allow(dead_code)]
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut res: i32 = 0;
        let mut cur: i32 = x;
        
        while cur != 0 {
            match res.checked_mul(10) {
                None => return 0,
                Some(tmp) => match tmp.checked_add(cur % 10) {
                    None => return 0,
                    Some(fine) => {
                        res = fine;
                    }
                } 
            }
            cur = cur / 10;
        }
        
        res
    }
}

fn main() {
    let x1 = 123;
    let x2 = -123;
    let x3 = 120;
    let x4 = -2147483648;

    println!("Input: x = {}", x1);
    println!("Output: {}", Solution::reverse(x1));

    println!("Input: x = {}", x2);
    println!("Output: {}", Solution::reverse(x2));

    println!("Input: x = {}", x3);
    println!("Output: {}", Solution::reverse(x3));

    println!("Input: x = {}", x4);
    println!("Output: {}", Solution::reverse(x4));
}
