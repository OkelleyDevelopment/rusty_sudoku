#![allow(warnings)]

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::process;

const SIZE: usize = 9;

fn main() {
    println!("\n- - - - - - - - - - - -");
    println!("| Rusty Sudoku Solver |");
    println!("- - - - - - - - - - - -\n");

    // Initialize the grid with zeros
    let mut grid: Vec<usize> = vec![0; SIZE * SIZE];
    //file input
    grid = read_file();
    println!(" The starting board:\n");

    display_board(&mut grid);

    if solve_board(&mut grid) == true {
        println!(" Solution found!\n");
        display_board(&mut grid);
    } else {
        println!(" No solution found.");
    }

    process::exit(0);
}

fn display_board(grid: &mut Vec<usize>) {
    for i in 0..SIZE {
        if i % 3 == 0 && i != 0 {
            print!("-------------------------\n");
        }
        for j in 0..SIZE {
            if j % 3 == 0 && j != 0 {
                print!(" | ");
            }

            if j == 8 {
                print!("{}\n", grid[i * SIZE + j]);
            }
            print!("{} ", grid[i * SIZE + j]);
        }
    }
    println!(" ");
}

fn remove_newlines(s: &mut String) {
    while s.ends_with('\n') || s.ends_with('\r') {
        s.pop();
    }
}

fn get_input() -> String {
    let mut input = String::new();

    while input == String::new() {
        io::stdin().read_line(&mut input);
    }
    return input;
}

// this function will read in an unsolved grid
// and then parse the file into a list of ints
fn read_file() -> Vec<usize> {
    io::stdout().flush().expect("Could not flush stdout");

    let mut grid: Vec<usize> = vec![];
    let mut filename;

    while let Err(_) = File::open(&filename) {
        println!("Please enter the board to solve: ");
        filename = get_input();
    }

    let mut file = File::open(filename).expect("Can't open the file!");
    let reader = BufReader::new(file).lines();

    for line in reader.lines() {
        let line = line.unwrap();
        let parsed = line.parse::<usize>().expect("Failed to parse line");
        grid.push(parsed);
    }
    return grid;
}

fn locate(board: &mut Vec<usize>, row: &mut usize, col: &mut usize) -> bool {
    for i in 0..SIZE {
        for j in 0..SIZE {
            if board[i * SIZE + j] == 0 {
                *row = i;
                *col = j;
                return true;
            }
        }
    }
    return false;
}

fn is_valid(board: &mut Vec<usize>, num: usize, pos: (usize, usize)) -> bool {
    // Check the row
    for i in 0..SIZE {
        if board[pos.0 * SIZE + i] == num && pos.1 != i {
            return false;
        }
    }
    // check the column
    for j in 0..SIZE {
        if board[j * SIZE + pos.1] == num && pos.0 != j {
            return false;
        }
    }
    // check the sub matrix
    let sub_x = pos.1 / 3;
    let sub_y = pos.0 / 3;

    for i in sub_y * 3..(sub_y * 3 + 3) {
        for j in sub_x * 3..(sub_x * 3 + 3) {
            if board[i * SIZE + j] == num && (i, j) != pos {
                return false;
            }
        }
    }

    return true;
}

// Solve the board
fn solve_board(board: &mut Vec<usize>) -> bool {
    let mut row: usize = 0;
    let mut col: usize = 0;
    let mut pos: (usize, usize) = (1000, 1000);

    // First new to find the first available cell
    let located: bool = locate(board, &mut row, &mut col);

    if (!located) {
        return true;
    }

    pos = (row, col);

    // Now find the next open cell and fill in the digit
    for n in 1..=9 {
        if is_valid(board, n, pos) == true {
            // if valid set the position to the number
            board[pos.0 * SIZE + pos.1] = n;
            // check if valid number is valid for the rest
            if solve_board(board) {
                return true;
            }

            // reset and backtrack
            board[pos.0 * SIZE + pos.1] = 0;
        }
    }
    return false;
}
