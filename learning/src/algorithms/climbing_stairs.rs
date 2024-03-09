// https://leetcode.com/problems/climbing-stairs/description/
// Recursive broke (TLE) at n = 45

#[allow(dead_code)]
struct Solution;

// Dynamic Programing approach:
// Runtime: 1ms - Beats 81.40% of users with Rust 
// Memory: 2.03mb - Beats 50.95% of users with Rust
#[allow(dead_code)]
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 1 {
            return 1;
        }

        let mut dp = vec![0; (n + 1) as usize];
        dp[0] = 1;
        dp[1] = 1;

        for i in 2..=n as usize {
            dp[i] = dp[i - 1] + dp[i - 2];
        }

        dp[n as usize]
    }
}

// TLE recursive approach
#[allow(dead_code)]
impl Solution {
    pub fn climb_stairs_recursive(n: i32) -> i32 {
        if n <= 2 {
            return n;
        }

        Self::climb_stairs(n - 1) + Self::climb_stairs(n - 2)
    }
}


fn main() {
    let main_time: std::time::Instant = std::time::Instant::now();


    let n1 = 2;
    let n2 = 45;

    println!("Ways to climb {} steps: {}", n1, Solution::climb_stairs(n1));
    println!("Ways to climb {} steps: {}", n2, Solution::climb_stairs(n2));

    
    // end of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\nExecution time: {:?} ({:?} ms)", duration, elapsed_ms);
}