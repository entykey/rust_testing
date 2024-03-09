fn main() {
    let main_time: std::time::Instant = std::time::Instant::now();


    let matrix: Vec<Vec<i32>> = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];

    if is_square(&matrix) {
        let diagonal_sum1 = sum_main_diagonal(&matrix);
        let diagonal_sum2 = sum_secondary_diagonal(&matrix);

        println!("Diagonal Sum 1: {}", diagonal_sum1);
        println!("Diagonal Sum 2: {}", diagonal_sum2);
    } else {
        println!("The matrix is not square.");
    }


    let matrix1: Vec<Vec<i32>> = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
    ];

    let matrix2: Vec<Vec<i32>> = vec![
        vec![7, 8],
        vec![9, 10],
        vec![11, 12],
    ];
    print_matrix_with_borders(&matrix1);

    // multiply
    if let Some(result) = matrix_multiply(&matrix1, &matrix2) {
        print_matrix(&result);
        print_matrix_with_borders(&result);
    } else {
        println!("Matrix multiplication is not possible.");
    }
    


    // end of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\nExecution time: {:?} ({:?} ms)", duration, elapsed_ms);
}

fn is_square(matrix: &Vec<Vec<i32>>) -> bool {
    let rows = matrix.len();
    if rows == 0 {
        return false;
    }
    let cols = matrix[0].len();
    rows == cols
}

fn sum_main_diagonal(matrix: &Vec<Vec<i32>>) -> i32 {
    matrix
        .iter()
        .enumerate()
        .fold(0, |sum, (i, row)| sum + row[i])
}

fn sum_secondary_diagonal(matrix: &Vec<Vec<i32>>) -> i32 {
    let size = matrix.len();
    matrix
        .iter()
        .enumerate()
        .fold(0, |sum, (i, row)| sum + row[size - 1 - i])
}

fn matrix_multiply(matrix1: &Vec<Vec<i32>>, matrix2: &Vec<Vec<i32>>) -> Option<Vec<Vec<i32>>> {
    let rows1 = matrix1.len();
    let cols1 = matrix1[0].len();
    let rows2 = matrix2.len();
    let cols2 = matrix2[0].len();

    if cols1 != rows2 {
        return None; // Matrix multiplication is not possible
    }

    let mut result = vec![vec![0; cols2]; rows1];

    for i in 0..rows1 {
        for j in 0..cols2 {
            for k in 0..cols1 {
                result[i][j] += matrix1[i][k] * matrix2[k][j];
            }
        }
    }

    Some(result)
}

fn print_matrix(matrix: &Vec<Vec<i32>>) {
    for row in matrix {
        for &val in row {
            print!("{} ", val);
        }
        println!();
    }
}
fn print_matrix_with_borders(matrix: &Vec<Vec<i32>>) {
    for row in matrix {
        print!("|");
        for &val in row {
            print!("{:^4}|", val);
        }
        println!();
    }
}
