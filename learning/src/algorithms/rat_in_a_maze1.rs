// inspired: https://stackoverflow.com/questions/9191428/maze-solving-algorithm-in-c
use std::thread;
use std::time::Duration;
use colored::*;

const MAZE_COLUMNS: usize = 9;
const MAZE_ROWS: usize = 9;

const WALL: char = '#';
const FREE: char = ' ';
const SOME_DUDE: char = '*';
const CORRECT_PATH: char = '+';
const STARTING_POINT: (usize, usize) = (1, 0);  // 1, 0
const ENDING_POINT: (usize, usize) = (8, 8);    // 7, 8

// no color
/*
fn print_the_maze(maze: &Vec<Vec<char>>) {
    for line in maze {
        let line_str: String = line.iter().collect();
        println!("{}", line_str);
    }
    println!();
}
*/
fn print_the_maze(maze: &Vec<Vec<char>>) {
    for line in maze {
        for &ch in line {
            let colored_char = match ch {
                '*' => ch.to_string().cyan(),
                '+' => ch.to_string().cyan(),
                _ => ch.to_string().normal(),
            };
            print!("{}", colored_char);
        }
        println!();
    }
    println!();
}


fn get(maze: &Vec<Vec<char>>, x: isize, y: isize) -> char {
    if x < 0 || y < 0 || x >= MAZE_COLUMNS as isize || y >= MAZE_ROWS as isize {
        WALL
    } else {
        maze[y as usize][x as usize]
    }
}

#[allow(dead_code)]
fn set(maze: &mut Vec<Vec<char>>, x: isize, y: isize, ch: char) {
    if x >= 0 && y >= 0 && x < MAZE_COLUMNS as isize && y < MAZE_ROWS as isize {
        maze[y as usize][x as usize] = ch;
    }
}

fn solve(maze: &mut Vec<Vec<char>>, x: isize, y: isize) -> bool {
    // check if reached the end as specified CONSTANTS
    if x == ENDING_POINT.0 as isize && y == ENDING_POINT.1 as isize {
        maze[y as usize][x as usize] = CORRECT_PATH;
        return true;
    }

    // check if not a possible path -> break
    if get(maze, x, y) != FREE {
        return false;
    }

    maze[y as usize][x as usize] = SOME_DUDE;

    println!("finding path attempt:");
    print_the_maze(maze);

    thread::sleep(Duration::from_millis(100)); // Add of 100ms

    // try to solve by directions: [ left, right, up, down ]
    if solve(maze, x - 1, y) || solve(maze, x + 1, y) || solve(maze, x, y - 1) || solve(maze, x, y + 1) {
        maze[y as usize][x as usize] = CORRECT_PATH;
        return true;
    }

    maze[y as usize][x as usize] = FREE;
    false
}

fn main() {
    let maze_data1: [&str; MAZE_ROWS] = [
        "# #######",
        "#   #   #",
        "# ### # #",
        "# #   # #",
        "# # # ###",
        "#   # # #",
        "# ### # #",
        "#   #   #",
        "####### #",
    ];

    let mut maze1: Vec<Vec<char>> = maze_data1.iter().map(|line| line.chars().collect()).collect();

    if solve(&mut maze1, STARTING_POINT.0 as isize, STARTING_POINT.1 as isize) {
        println!("Solved!");
    } else {
        println!("Cannot solve. :-(");
    }
    print_the_maze(&maze1);



    println!("solving maze_data2 in 2s...");
    thread::sleep(Duration::from_secs(2));

    let maze_data2: [&str; MAZE_ROWS] = [
        "# #######",
        "#   #   #",
        "# ### # #",
        "# #   # #",
        "# # ### #",
        "#   # ###",
        "# # # # #",
        "#      ##",    // <- blocked
        "####### #",
    ];

    let mut maze2: Vec<Vec<char>> = maze_data2.iter().map(|line| line.chars().collect()).collect();

    if solve(&mut maze2, STARTING_POINT.0 as isize, STARTING_POINT.1 as isize) {
        println!("Solved!");
    } else {
        println!("Cannot solve. :-(");
    }
    print_the_maze(&maze2);
}
