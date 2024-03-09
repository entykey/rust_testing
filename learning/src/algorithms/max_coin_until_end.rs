// https://www.geeksforgeeks.org/collect-maximum-coins-before-hitting-a-dead-end/?ref=ml_lbp

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    fn is_valid(i: i32, j: i32, m: i32, n: i32) -> bool {
        i >= 0 && i < m && j >= 0 && j < n
    }

    fn max_coins_util(
        board: &Vec<Vec<char>>,
        i: i32,
        j: i32,
        dir: i32,
        dp: &mut Vec<Vec<[i32; 2]>>,
    ) -> i32 {
        let m = board.len() as i32;
        let n = board[0].len() as i32;

        if !Self::is_valid(i, j, m, n) || board[i as usize][j as usize] == '#' {
            return 0;
        }

        if dp[i as usize][j as usize][dir as usize] != -1 {
            return dp[i as usize][j as usize][dir as usize];
        }

        dp[i as usize][j as usize][dir as usize] =
            if board[i as usize][j as usize] == 'C' {
                1
            } else {
                0
            };

        if dir == 1 {
            dp[i as usize][j as usize][dir as usize] += std::cmp::max(
                Self::max_coins_util(board, i + 1, j, 0, dp),
                Self::max_coins_util(board, i, j + 1, 1, dp),
            );
        }

        if dir == 0 {
            dp[i as usize][j as usize][dir as usize] += std::cmp::max(
                Self::max_coins_util(board, i + 1, j, 1, dp),
                Self::max_coins_util(board, i, j - 1, 0, dp),
            );
        }

        dp[i as usize][j as usize][dir as usize]
    }

    fn max_coins(board: Vec<Vec<char>>) -> i32 {
        let m = board.len() as i32;
        let n = board[0].len() as i32;
        let mut dp = vec![vec![[-1; 2]; n as usize]; m as usize];

        Self::max_coins_util(&board, 0, 0, 1, &mut dp)
    }
}

/*
'C' -->  This cell has coin

'#' -->  This cell is a blocking cell. 
         We can not go anywhere from this.

'E' -->  This cell is empty. We don't get
         a coin, but we can move from here.  
*/
fn main() {
    let main_time: std::time::Instant = std::time::Instant::now();

    let board: Vec<Vec<char>> = vec![
        vec!['E', 'C', 'C', 'C', 'C'],
        vec!['C', '#', 'C', '#', 'E'],
        vec!['#', 'C', 'C', '#', 'C'],
        vec!['C', 'E', 'E', 'C', 'E'],
        vec!['C', 'E', '#', 'C', 'E'],
    ];

    println!("Maximum number of collected coins is {}", Solution::max_coins(board));

    // end of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\nExecution time: {:?} ({:?} ms)", duration, elapsed_ms);
}
