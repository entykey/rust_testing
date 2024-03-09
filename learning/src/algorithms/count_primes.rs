// https://leetcode.com/problems/count-primes/
// Sieve of Eratosthenes algorithm approach

#[allow(dead_code)]
struct Solution1;
struct Solution2;

// Runtime: 67ms - Beats 61.36%of users with Rust
// Memory: 6.79mb - Beats 70.45%of users with Rust
impl Solution1 {
    pub fn count_primes(n: i32) -> i32 {
        let n = n as usize;
        if n <= 2 {
            return 0;
        }
        let sqrted_n = (n as f64).sqrt() as usize;
        let mut validations: Vec<bool> = vec![true; n];
        validations[0] = false;
        validations[1] = false;
        for i in 2..=sqrted_n {
            if !validations[i] {
                continue;
            } else {
                validations[i] = true;
                let mut j = i * i;
                while j < n {
                    validations[j] = false;
                    j += i;
                }
            }
        }
        validations.into_iter().filter(|&a| a).count() as i32
    }
}



// almost best solution ( C++ took 100+ ms in Runtime and 10mb + mb in Memory )
// Runtime: 73ms - Beats 50.00% of users with Rust
// Memory: 2.64mb - Beats 93.18% of users with Rust
#[allow(dead_code)]
impl Solution2 {
    pub fn count_primes(n: i32) -> i32 {
        let n = n as usize;
        if n < 8 {
            return [0, 0, 0, 1, 2, 2, 3, 3][n];
        }
        let mut sieve = vec![0_u8; ((n - 1) >> 3) + 1];
        let inc = [6, 4, 2, 4, 2, 4, 6, 2];
        let mut i = 1;
        let mut answer = 3;
        for k in 0.. {
            i += inc[k & 7];
            if i >= n {
                break;
            }
            if sieve[i >> 3] & 1 << (i & 7) == 0 {
                answer += 1;
                let mut j = i * i;
                while j < n {
                    sieve[j >> 3] |= 1 << (j & 7);
                    j += i;
                }
            }
        }
        answer
    }
}

#[allow(dead_code)]
fn count_primes(n: i32) -> i32 {
    if n <= 2 {
        return 0;
    }

    let n = n as usize;
    let mut count = 0;
    let mut is_prime = vec![true; n];

    // 0 and 1 are not prime
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..(n as f64).sqrt() as usize + 1 {
        if is_prime[i] {
            for j in (i * i..n).step_by(i) {
                is_prime[j] = false;
            }
        }
    }

    for i in 2..n {
        if is_prime[i] {
            count += 1;
        }
    }

    count
}

#[allow(dead_code)]
fn try_u64_to_usize(value: u64) -> Option<usize> {
    if value <= usize::MAX as u64 {
        Some(value as usize)
    } else {
        None
    }
}

// not even good as the conventional for loop approach
#[allow(dead_code)]
fn count_primes_bitset(n: i32) -> i32 {
    if n <= 2 {
        return 0;
    }

    let n = n as u64;
    let mut count = 0;
    let mut is_prime = vec![true; (n + 1) as usize];

    for i in 2..n {
        if is_prime[i as usize] {
            count += 1;
            let mut j = i * i;
            while j <= n {
                is_prime[j as usize] = false;
                j += i;
            }
        }
    }

    count

    
    // with specific type instead of usize:  (same performance lol)
    /*
    if n <= 2 {
        return 0;
    }

    let n = n as u64;
    let mut count = 1; // Account for 2 as the only even prime number

    // Calculate the size of the is_prime vector
    let is_prime_size = try_u64_to_usize((n + 1) / 2).expect("Input is too large");

    let mut is_prime = vec![true; is_prime_size];

    for i in (3..=n).step_by(2) {
        let idx = try_u64_to_usize(i / 2).expect("Input is too large");
        if idx >= is_prime.len() {
            break;
        }

        if is_prime[idx] {
            count += 1;

            let mut j = i * i;
            while j <= n {
                let j_idx = try_u64_to_usize(j / 2).expect("Input is too large");
                if j_idx < is_prime.len() {
                    is_prime[j_idx] = false;
                } else {
                    break;
                }

                j += i * 2;
            }
        }
    }

    count
    */

}


fn main() {
    let main_time: std::time::Instant = std::time::Instant::now();



    let n = 10; // Replace this with any value of 'n' you want to test
    let count = Solution1::count_primes(n);
    println!("Number of prime numbers less than {}: {}", n, count);



    // end of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\nExecution time: {:?} ({:?} ms)", duration, elapsed_ms);
}