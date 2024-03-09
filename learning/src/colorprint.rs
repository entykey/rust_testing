use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use std::io::Write;

const MAZE_COLUMNS: usize = 9;
const MAZE_ROWS: usize = 9;

const WALL: char = '#';
const FREE: char = ' ';
const SOME_DUDE: char = '*';
const CORRECT_PATH: char = '+';
const STARTING_POINT: (usize, usize) = (1, 0);
const ENDING_POINT: (usize, usize) = (7, 8);

fn print_da_maze(maze: &Vec<Vec<char>>) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let mut color_spec = ColorSpec::new();

    for line in maze {
        for &ch in line {
            match ch {
                '*' => {
                    color_spec.set_fg(Some(Color::Cyan));
                    stdout.set_color(&color_spec).unwrap();
                    print!("{}", ch);
                    stdout.reset().unwrap();
                }
                '+' => {
                    color_spec.set_fg(Some(Color::Cyan));
                    stdout.set_color(&color_spec).unwrap();
                    print!("{}", ch);
                    stdout.reset().unwrap();
                }
                _ => print!("{}", ch),
            }
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

fn set(maze: &mut Vec<Vec<char>>, x: isize, y: isize, ch: char) {
    if x >= 0 && y >= 0 && x < MAZE_COLUMNS as isize && y < MAZE_ROWS as isize {
        maze[y as usize][x as usize] = ch;
    }
}

fn solve(maze: &mut Vec<Vec<char>>, x: isize, y: isize) -> bool {
    if x == ENDING_POINT.0 as isize && y == ENDING_POINT.1 as isize {
        maze[y as usize][x as usize] = CORRECT_PATH;
        return true;
    }

    if get(maze, x, y) != FREE {
        return false;
    }

    maze[y as usize][x as usize] = SOME_DUDE;

    println!("finding path attempt:");
    print_da_maze(maze);

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

    // Additional empty line to separate mazes
    println!();

    let maze_data2: [&str; MAZE_ROWS] = [
        "# #######",
        "#   #   #",
        "# ### # #",
        "# #   # #",
        "# # # ###",
        "#   # # #",
        "# ### # #",
        "#   ##  #",
        "####### #",
    ];

    let mut maze2: Vec<Vec<char>> = maze_data2.iter().map(|line| line.chars().collect()).collect();

    if solve(&mut maze2, STARTING_POINT.0 as isize, STARTING_POINT.1 as isize) {
        println!("Solved!");
    } else {
        println!("Cannot solve. :-(");
    }
}
