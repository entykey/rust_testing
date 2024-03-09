#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
// Runtime: 3ms - Beats 77.78%of users with Rust
// Memory: 2.08mb - Beats 79.63%of users with Rust
impl BitsetsSolution for Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut rows = vec![0; 9];
        let mut cols = vec![0; 9];
        let mut boxes = vec![0; 9];

        // Fill the bitsets with the existing digits
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] != '.' {
                    let digit = board[i][j] as u8 - b'1';
                    let mask = 1 << digit;
                    rows[i] |= mask;
                    cols[j] |= mask;
                    boxes[(i / 3) * 3 + (j / 3)] |= mask;
                }
            }
        }

        Self::solve(board, &mut rows, &mut cols, &mut boxes);
    }

    fn is_valid(row: usize, col: usize, digit: u8, rows: &Vec<i32>, cols: &Vec<i32>, boxes: &Vec<i32>) -> bool {
        let mask = 1 << digit;
        (rows[row] & mask == 0) && (cols[col] & mask == 0) && (boxes[(row / 3) * 3 + (col / 3)] & mask == 0)
    }

    fn solve(board: &mut Vec<Vec<char>>, rows: &mut Vec<i32>, cols: &mut Vec<i32>, boxes: &mut Vec<i32>) -> bool {
        for row in 0..9 {
            for col in 0..9 {
                if board[row][col] == '.' {
                    for digit in 0..9 {
                        if Self::is_valid(row, col, digit, rows, cols, boxes) {
                            let digit_char = (digit as u8 + b'1') as char;
                            let mask = 1 << digit;
                            board[row][col] = digit_char;
                            rows[row] |= mask;
                            cols[col] |= mask;
                            boxes[(row / 3) * 3 + (col / 3)] |= mask;

                            if Self::solve(board, rows, cols, boxes) {
                                return true;
                            }

                            board[row][col] = '.';
                            rows[row] &= !mask;
                            cols[col] &= !mask;
                            boxes[(row / 3) * 3 + (col / 3)] &= !mask;
                        }
                    }
                    return false; // All digits tried and no valid placement
                }
            }
        }
        true // Board is filled with valid digits
    }
}


/*
// Runtime: 6ms - Beats 45.37%of users with Rust
// Memory: 2.21mb - Beats 19.44%of users with Rust
impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        Self::solve(board);
    }

    fn is_valid(board: &Vec<Vec<char>>, row: usize, col: usize, num: char) -> bool {
        for i in 0..9 {
            if board[i][col] == num || board[row][i] == num {
                return false;
            }
        }

        let start_row = 3 * (row / 3);
        let start_col = 3 * (col / 3);
        for i in 0..3 {
            for j in 0..3 {
                if board[start_row + i][start_col + j] == num {
                    return false;
                }
            }
        }

        true
    }

    fn solve(board: &mut Vec<Vec<char>>) -> bool {
        for row in 0..9 {
            for col in 0..9 {
                if board[row][col] == '.' {
                    for ch in ['1', '2', '3', '4', '5', '6', '7', '8', '9'] {
                        if Self::is_valid(board, row, col, ch) {
                            board[row][col] = ch;
                            if Self::solve(board) {
                                return true;
                            }
                            board[row][col] = '.'; // Backtrack if the current placement is not valid
                        }
                    }
                    return false; // All digits tried and no valid placement
                }
            }
        }
        true // Board is filled with valid digits
    }
}
*/


/* C++ solution:
// Runtime: 7ms Beats 97.90% of users with C++
// Memory: 6.54mb Beats 10.71% of users with C++
class Solution {
private:
    void ans(vector<vector<char>>& board, vector<vector<char>>& curr, int i, int j, vector<vector<bool>>& col, vector<vector<bool>>& row, vector<vector<bool>>& mat) {
        // Find the next empty cell
        while(i < 9 && board[i][j] != '.') {
            if (j < 8)
                j++;
            else {
                j = 0;
                i++;
            }
        }

        // If we have reached the end of the board, we have found a solution
        if(i == 9) {
            curr = board;
            return;
        }

        // Try different numbers from 1 to 9
        for(int k = 1; k <= 9; k++) {
            // If a solution has already been found, exit the loop
            if(curr.size() > 0)
                break;

            // Check if the current number k is valid for the current cell
            if(!col[j][k] && !row[i][k] && !mat[3*(i/3) + (j/3)][k]) {
                // Mark the current number k as used
                col[j][k] = true;
                row[i][k] = true;
                mat[3*(i/3) + (j/3)][k] = true;

                // Place the current number k in the current cell
                board[i][j] = (char)('0'+k);

                // Recursively solve the Sudoku puzzle
                ans(board, curr, i, j, col, row, mat);

                // Undo the current number k and mark it as unused
                col[j][k] = false;
                row[i][k] = false;
                mat[3*(i/3) + (j/3)][k] = false;
                board[i][j] = '.';
            }
        }
    }
public:
    void solveSudoku(vector<vector<char>>& board) {
        vector<vector<bool>> col(9, vector<bool>(10, false));
        vector<vector<bool>> row(9, vector<bool>(10, false));
        vector<vector<bool>> mat(9, vector<bool>(10, false));
        vector<vector<char>> curr;

        for(int i = 0; i < 9; i++) {
            for(int j = 0; j < 9; j++) {
                if(board[i][j] != '.') {
                    int x = board[i][j] - '0';
                    row[i][x] = true;
                    col[j][x] = true;
                    mat[3*(i/3) + (j/3)][x] = true;
                }
            }
        }

        ans(board, curr, 0, 0, col, row, mat);
        board = curr;
        return;
    }
};
*/

fn main() {
    let main_time: std::time::Instant = std::time::Instant::now();

    let mut board = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];

    Solution::solve_sudoku(&mut board);

    // Print the solved board
    for row in &board {
        println!("{:?}", row);
    }

    // end of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\nExecution time: {:?} ({:?} ms)", duration, elapsed_ms);
}
