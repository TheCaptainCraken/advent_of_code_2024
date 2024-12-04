use std::{fs, io::Result};

const INPUT: &str = "data/day_04/input.txt";

fn main() -> Result<()> {
    let file = fs::read_to_string(INPUT)?;

    let input: Vec<Vec<char>> = file.lines().map(|line| line.chars().collect()).collect();
    let rows = input.len();
    let columns = input[0].len();

    let mut counter_first_part = 0;
    let mut counter_second_part = 0;

    for row in 0..rows {
        for column in 0..columns {
            if input[row][column] == 'X' {
                let pos = (row, column);
                if check_right(pos, &input) {
                    counter_first_part += 1;
                }
                if check_left(pos, &input) {
                    counter_first_part += 1;
                }
                if check_up(pos, &input) {
                    counter_first_part += 1;
                }
                if check_down(pos, &input) {
                    counter_first_part += 1;
                }
                if check_diagonal_left_down(pos, &input) {
                    counter_first_part += 1;
                }
                if check_diagonal_left_up(pos, &input) {
                    counter_first_part += 1;
                }
                if check_diagonal_right_up(pos, &input) {
                    counter_first_part += 1;
                }
                if check_diagonal_right_down(pos, &input) {
                    counter_first_part += 1;
                }
            } else if input[row][column] == 'A' {
                let pos = (row, column);
                if check_x_mas(pos, &input) {
                    counter_second_part += 1;
                }
            }
        }
    }

    println!("First part: {}", counter_first_part);
    println!("Second part: {}", counter_second_part);
    Ok(())
}

fn check_right(pos: (usize, usize), input: &Vec<Vec<char>>) -> bool {
    let (row, column) = pos;
    let columns = input[0].len();

    if column + 3 < columns {
        if input[row][column + 1] == 'M'
            && input[row][column + 2] == 'A'
            && input[row][column + 3] == 'S'
        {
            return true;
        }
    }

    return false;
}

fn check_left(pos: (usize, usize), input: &Vec<Vec<char>>) -> bool {
    let (row, column) = pos;

    if column >= 3 {
        if input[row][column - 1] == 'M'
            && input[row][column - 2] == 'A'
            && input[row][column - 3] == 'S'
        {
            return true;
        }
    }

    return false;
}

fn check_up(pos: (usize, usize), input: &Vec<Vec<char>>) -> bool {
    let (row, column) = pos;

    if row >= 3 {
        if input[row - 1][column] == 'M'
            && input[row - 2][column] == 'A'
            && input[row - 3][column] == 'S'
        {
            return true;
        }
    }

    return false;
}

fn check_down(pos: (usize, usize), input: &Vec<Vec<char>>) -> bool {
    let (row, column) = pos;
    let rows = input.len();

    if row + 3 < rows {
        if input[row + 1][column] == 'M'
            && input[row + 2][column] == 'A'
            && input[row + 3][column] == 'S'
        {
            return true;
        }
    }

    return false;
}

fn check_diagonal_left_down(pos: (usize, usize), input: &Vec<Vec<char>>) -> bool {
    let (row, column) = pos;
    let rows = input.len();

    if row + 3 < rows && column >= 3 {
        if input[row + 1][column - 1] == 'M'
            && input[row + 2][column - 2] == 'A'
            && input[row + 3][column - 3] == 'S'
        {
            return true;
        }
    }

    return false;
}

fn check_diagonal_left_up(pos: (usize, usize), input: &Vec<Vec<char>>) -> bool {
    let (row, column) = pos;

    if row >= 3 && column >= 3 {
        if input[row - 1][column - 1] == 'M'
            && input[row - 2][column - 2] == 'A'
            && input[row - 3][column - 3] == 'S'
        {
            return true;
        }
    }

    return false;
}

fn check_diagonal_right_up(pos: (usize, usize), input: &Vec<Vec<char>>) -> bool {
    let (row, column) = pos;
    let columns = input[0].len();

    if row >= 3 && column + 3 < columns {
        if input[row - 1][column + 1] == 'M'
            && input[row - 2][column + 2] == 'A'
            && input[row - 3][column + 3] == 'S'
        {
            return true;
        }
    }

    return false;
}

fn check_diagonal_right_down(pos: (usize, usize), input: &Vec<Vec<char>>) -> bool {
    let (row, column) = pos;
    let rows = input.len();
    let columns = input[0].len();

    if row + 3 < rows && column + 3 < columns {
        if input[row + 1][column + 1] == 'M'
            && input[row + 2][column + 2] == 'A'
            && input[row + 3][column + 3] == 'S'
        {
            return true;
        }
    }

    return false;
}

/*
M.S
.A.
M.S
*/

fn check_x_mas(pos: (usize, usize), input: &Vec<Vec<char>>) -> bool {
    let (row, column) = pos;
    let rows = input.len();
    let columns = input[0].len();

    if row + 1 < rows && column + 1 < columns && row > 0 && column > 0 {
        let upper_diagonal = (input[row - 1][column - 1] == 'M'
            && input[row + 1][column + 1] == 'S')
            || (input[row - 1][column - 1] == 'S' && input[row + 1][column + 1] == 'M');
        let lower_diagonal = (input[row - 1][column + 1] == 'S'
            && input[row + 1][column - 1] == 'M')
            || (input[row - 1][column + 1] == 'M' && input[row + 1][column - 1] == 'S');
        return upper_diagonal && lower_diagonal;
    }

    return false;
}
