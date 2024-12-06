use std::{fs, io::Result};

const INPUT: &str = "data/day_06/input.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PieceType {
    Empty,
    Wall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Position {
    fn traslate(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }

    fn next(&self, direction: Direction) -> Self {
        let mut new_position = self.clone();
        new_position.traslate(direction);
        new_position
    }
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

fn main() -> Result<()> {
    let file = fs::read_to_string(INPUT)?;

    let mut grid = Vec::new();

    let mut position = Position::new(0, 0);
    let mut current_direction = Direction::Up;

    for (ridx, line) in file.lines().enumerate() {
        let mut row = Vec::new();
        for (cidx, c) in line.chars().enumerate() {
            match c {
                '.' => row.push(PieceType::Empty),
                '#' => row.push(PieceType::Wall),
                '^' => {
                    row.push(PieceType::Empty);
                    position = Position::new(cidx as i32, ridx as i32);
                }
                _ => panic!("Invalid character in input"),
            }
        }
        grid.push(row);
    }

    let max_col = grid[0].len() as i32;
    let max_row = grid.len() as i32;

    let mut visited_positions = Vec::from([position]);
    loop {
        let next_position = position.next(current_direction);
        if next_position.x < 0
            || next_position.x >= max_col
            || next_position.y < 0
            || next_position.y >= max_row
        {
            break;
        }
        if grid[next_position.y as usize][next_position.x as usize] == PieceType::Wall {
            current_direction = match current_direction {
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
            }
        }

        position.traslate(current_direction);
        if !visited_positions.contains(&position) {
            visited_positions.push(position);
        }
    }

    println!("The guard made {} moves", visited_positions.len());

    Ok(())
}
