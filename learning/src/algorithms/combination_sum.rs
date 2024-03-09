// Combination Sum solution

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
// Runtime: -ms - Beats 100.00% of users with Rust
// Memory: 2.08mb - Beats 78.85% of users with Rust
impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut path = Vec::new();

        // Sort the candidates to easily avoid duplicates
        let mut candidates = candidates;
        candidates.sort();

        fn backtrack(
            candidates: &Vec<i32>,
            target: i32,
            start: usize,
            path: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            if target == 0 {
                // Found a combination that sums up to the target
                result.push(path.clone());
                return;
            }

            for i in start..candidates.len() {
                if i > start && candidates[i] == candidates[i - 1] {
                    // Skip duplicate elements to avoid duplicates in the result
                    continue;
                }

                let current = candidates[i];
                if current > target {
                    // If the current number is greater than the target, no need to explore further
                    break;
                }

                // Choose the current number as part of the combination
                path.push(current);
                // Recur with the remaining target and the next index to avoid using the same element twice
                backtrack(candidates, target - current, i + 1, path, result);
                // Backtrack: remove the current number from the path to explore other possibilities
                path.pop();
            }
        }

        // Start the backtracking process
        backtrack(&candidates, target, 0, &mut path, &mut result);
        result
    }
}


fn main() {
    let candidates1 = vec![10, 1, 2, 7, 6, 1, 5];
    let target1 = 8;
    println!("{:?}", Solution::combination_sum2(candidates1, target1));

    let candidates2 = vec![2, 5, 2, 1, 2];
    let target2 = 5;
    println!("{:?}", Solution::combination_sum2(candidates2, target2));
}
