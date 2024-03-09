// https://leetcode.com/problems/set-matrix-zeroes/description/

#[allow(dead_code)]
struct Solution;
struct Solution2;
struct Solution3;

// regular for loop solution:
// Runtime: 3ms - Beats 93.55% of users with Rust
// Memory: 2.37mb - Beats 75.48% of users with Rust
#[allow(dead_code)]
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();    // row (total of sub vecs in the board vec)
        let n = matrix[0].len(); // number of elements in the sub vec

        let mut first_col_zero = false;

        // Step 1: Mark the first element of each row and column as 0 whenever a 0 is encountered
        for i in 0..m {
            if matrix[i][0] == 0 {
                first_col_zero = true;
            }

            // each elm in a row:
            for j in 1..n {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;   // elm '0' detected at the i row, set first elm of the row to '0' (elm at row i, col 0)
                    matrix[0][j] = 0;   // set first elm of the col to '0' (elm at row 0, col j)
                }
            }
        }
        // now: [1, 0, 1]
        //      [0, 0, 1]
        //      [1, 1, 1]

        // Step 2: Set entire rows and columns to 0 based on the first element of each row and column
        for i in 1..m {
            for j in 1..n {
                if matrix[i][0] == 0 || matrix[0][j] == 0 { // check all elm in top & left corner
                    matrix[i][j] = 0;
                }
            }
        }
        // now: [1, 0, 1]
        //      [0, 0, 0]
        //      [1, 0, 1]

        // but if there's also a '0' at the top left
        // ->   [0, 0, 1]
        //      [0, 0, 0]
        //      [1, 0, 1]

        // Handle the first row and column separately

        // set entire row to '0':
        if matrix[0][0] == 0 {
            for j in 0..n {
                matrix[0][j] = 0;
            }
        }

        // set entire col to '0'
        if first_col_zero {
            for i in 0..m {
                matrix[i][0] = 0;
            }
        }

    }
}


// use the matrix itself to keep track of the rows and columns that need to be zeroed.
// (sometimes 6ms, somtimes 0ms)
// Runtime: 6ms - Beats 100.00% of users with Rust
// Memory: 2.36mb - Beats 75.48% of users with Rust
#[allow(dead_code)]
impl Solution2 {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut first_row_zero = false;
        let mut first_col_zero = false;

        // Step 1: Mark rows and columns to be zeroed in the first row and first column.
        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 0 {
                    if i == 0 {
                        first_row_zero = true;
                    }
                    if j == 0 {
                        first_col_zero = true;
                    }
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }

        // Step 2: Set elements to zero based on markers in the first row and first column.
        for i in 1..m {
            for j in 1..n {
                if matrix[i][0] == 0 || matrix[0][j] == 0 {
                    matrix[i][j] = 0;
                }
            }
        }

        // Step 3: Update the first row and first column based on the boolean variables.
        if first_row_zero {
            for j in 0..n {
                matrix[0][j] = 0;
            }
        }

        if first_col_zero {
            for i in 0..m {
                matrix[i][0] = 0;
            }
        }
    }
}



// Runtime: 6ms - Beats 63.23% of users with Rust
// Memory: 2.31mb - Beats 75.48% of users with Rust
#[allow(dead_code)]
impl Solution3 {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut zero_column = false;
        let mut zero_row = false;

        for r in 0..m {
            for c in 0..n {
                if matrix[r][c] == 0 {
                    if c == 0 {
                        zero_column = true;
                    } else {
                        matrix[0][c] = 0;
                    }

                    if r == 0 {
                        zero_row = true;
                    } else {
                        matrix[r][0] = 0;
                    }
                }
            }
        }

        for r in 1..m {
            for c in 1..n {
                if matrix[0][c] == 0 || matrix[r][0] == 0 {
                    matrix[r][c] = 0;
                }
            }
        }

        if zero_column {
            for r in 0..m {
                matrix[r][0] = 0;
            }
        }

        if zero_row {
            for c in 0..n {
                matrix[0][c] = 0;
            }
        }
    }
}



// Go solution:
// Runtime: 11ms - Beats 78.18%of users with Go
// Memory: 6.45mb - Beats 71.92%of users with Go
/*
func setZeroes(matrix [][]int)  {
    m, n := len(matrix), len(matrix[0])
    zeroColumn, zeroRow := false, false
    for r := 0; r < m; r++ {
        for c := 0; c < n; c++ {
            if matrix[r][c] == 0 {
                if c == 0 { zeroColumn = true } else { matrix [0][c] = 0 }
                if r == 0 { zeroRow = true } else { matrix [r][0] = 0 }
            }
        }
    }
    for r := 1; r < m; r++ {
        for c := 1; c < n; c++ {
            if matrix[0][c] == 0 || matrix[r][0] == 0 {
                matrix[r][c] = 0
            }
        }
    }
    if zeroColumn { for r := 0; r < m; r++ { matrix[r][0] = 0 } }
    if zeroRow { for c := 0; c < n; c++ { matrix[0][c] = 0 } }
}
*/


fn main() {
    let main_time: std::time::Instant = std::time::Instant::now();

    let mut matrix: Vec<Vec<i32>> = vec![
        vec![0, 1, 1],
        vec![1, 0, 1],
        vec![1, 1, 1]
    ];

    // vector looping tutorial examples:
    /*
    for (i, row) in matrix.iter_mut().enumerate() {
        for (y, col) in row.iter_mut().enumerate() {
            println!("{}", col);
        }
    }
    println!();
    for row in &matrix {
        print!("[");
        for (i, &col) in row.iter().enumerate() {
            print!("{}", col);
            if i < row.len() - 1 {
                print!(", ");
            }
        }
        println!("]");
    }
    println!();
    for row in &matrix {
        print!("[");
        for (i, col) in row.iter().enumerate() {
            if i > 0 {
                print!(", ");
            }
            print!("{}", col);
        }
        println!("]");
    }
    println!();
    for row in &matrix {
        for col in row {
            print!("{} ", col);
        }
        println!();
    }

    let mut sum = 0;
    for row in &matrix {
        for &col in row {
            sum += col;
        }
    }

    println!("Sum of elements: {}", sum);

    sum = 0;
    for (i, row) in matrix.iter().enumerate() {
        for (j, &col) in row.iter().enumerate() {
            println!("matrix[{}][{}] = {}", i, j, col);
            sum += col;
        }
    }

    println!("Sum of elements: {}", sum);

    for row in &matrix {
        for &col in row {
            if col == 0{

            }
        }
    }
    */

    Solution2::set_zeroes(&mut matrix);

    println!();
    for row in &matrix {
        print!("[");
        for (i, col) in row.iter().enumerate() {
            if i > 0 {
                print!(", ");
            }
            print!("{}", col);
        }
        println!("]");
    }

    // end of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\nExecution time: {:?} ({:?} ms)", duration, elapsed_ms);
}
