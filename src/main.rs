#![allow(warnings)]

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::process;

// Constant representing the size of the board (X x Y)
const SIZE: usize = 9;

/**
 * The main function or driver of the program. Initializes the
 * mutable board variable and then calls the functions to attempt
 * solving the board.
 *
 * Arguments:
 *      None
 *
 * Returns:
 *      None
 */
fn main() {
    println!("\n- - - - - - - - - - - -");
    println!("| Rusty Sudoku Solver |");
    println!("- - - - - - - - - - - -\n");

    // Initialize the grid with zeros
    //let mut board: Vec<usize> = vec![0; SIZE * SIZE];
    //file input
    let filename = String::from("./unsolved/") + &get_input();
    let mut board = read_file(filename);
    println!("\nThe starting board:");

    display_board(&mut board);

    if solve_board(&mut board) == true {
        println!("\nSolution found!");
        display_board(&mut board);
    } else {
        println!("No solution found.");
    }
    process::exit(0);
}

/**
 * A function to display the board.
 *
 * Arguments:
 *      grid - the game board being printed
 *
 * Returns:
 *      None
 */
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
                print!("{}", grid[i * SIZE + j]);
            } else {
                print!("{} ", grid[i * SIZE + j]);
            }
            io::stdout().flush().expect("Could not flush");
        }
        println!(" ");
    }
}

/**
 * A function to get user input for the filename of the sudoku
 * board.
 *
 * Arguments:
 *      None
 *
 * Returns:
 *      String - the filename
 *
 */
fn get_input() -> String {
    println!("Pleae enter the file name for the board: ");
    let mut input = String::new();

    while input == String::new() {
        io::stdin()
            .read_line(&mut input)
            .expect("Error with reading input");
    }
    return input.trim().to_string();
}

/**
 * This function will read in an unsolved board and
 * then parse the file into a Vector representing the
 * board for the program.
 *
 * Arguments:
 *      filename - A mutable reference to a String containing the file
 *                 name for the sudoku board
 *
 * Returns:
 *      Vec<usize> - The game board
 */
fn read_file(mut filename: String) -> Vec<usize> {
    let mut grid: Vec<usize> = vec![];
    let mut file = File::open(&filename);

    while file.is_err() {
        filename = get_input();
        file = File::open(&filename);
    }
    let reader = BufReader::new(file.unwrap());

    for line in reader.lines() {
        let line = line.unwrap();
        let chars: Vec<char> = line.chars().collect();
        for c in chars {
            if !c.is_whitespace() {
                grid.push(c.to_digit(10).unwrap() as usize);
            }
        }
    }
    return grid;
}

/**
 * A function to find the next position in the board to fill in.
 *
 * Arguments:
 *      board - A mutable reference to a Vector representing the board
 *      row - A mutable reference to the row index
 *      col - A mutable reference to the column index
 *
 * Updates:
 *      row - Updates the value of the row variable
 *      col - Updates the value of the column variable
 *
 * Return:
 *      bool - True or False if a spot is found
 *
 */
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

/**
 * A function that checks if the current value is a valid valuue.
 *
 * Arguments:
 *      board - A mutable reference to a Vector representing the board
 *      num - The current value being checked
 *      pos - a tuple representing the (X,Y) position in the board.
 *
 * Return:
 *      bool - True or False depending on if the num is value
 */
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

/**
 * A function that uses a backtracking algorithm while
 * attempting to solve a sudoku board
 *
 * Arguments:
 *      board - A mutable reference to a Vector representing the board
 *
 * Returns:
 *      bool - True or False depending on whether the board is solved.
 */
fn solve_board(board: &mut Vec<usize>) -> bool {
    // Variable for the row index
    let mut row: usize = 0;
    // Variable for the column index
    let mut col: usize = 0;
    // The tuple for the (x,y) tuple
    let mut pos: (usize, usize) = (1000, 1000);

    // First new to find the first available cell
    let located: bool = locate(board, &mut row, &mut col);

    if (!located) {
        return true;
    }
    // Set the position
    pos = (row, col);

    // Now find the next open cell and fill in the digit
    for n in 1..=9 {
        if is_valid(board, n, pos) == true {
            // If valid set the position to the number
            board[pos.0 * SIZE + pos.1] = n;
            // Check if valid number is valid for the rest
            if solve_board(board) {
                return true;
            }
            // Reset and backtrack
            board[pos.0 * SIZE + pos.1] = 0;
        }
    }
    // Return false if board is not solveable currently
    return false;
}
