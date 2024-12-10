use std::collections::HashSet;

const INPUT: &str = include_str!("../data/day_10/input.txt");

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    const DIRECTIONS: [Direction; 4] = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];
}

fn main() {
    let parsed_input = INPUT
        .lines()
        .map(|line| {
            line.chars()
                .map(|character| character.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let starting_positions = parsed_input
        .iter()
        .enumerate()
        .flat_map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &value)| value == 0)
                .map(move |(column_index, _)| (row_index as u32, column_index as u32))
        })
        .collect::<Vec<_>>();

    let mut part_one_answer = 0;
    let mut part_two_answer = 0;
    for start_position in starting_positions {
        let legal_paths_part_1 = identify_legal_paths_part_1(&parsed_input, start_position, 0);
        let legal_paths_part_2 = identify_legal_paths_part_2(&parsed_input, start_position, 0);

        part_one_answer += legal_paths_part_1.len();
        part_two_answer += legal_paths_part_2;
    }

    println!("First part: {}", part_one_answer);
    println!("Second part: {}", part_two_answer);
}

fn identify_legal_paths_part_1(
    map: &Vec<Vec<u32>>,
    start_position: (u32, u32),
    current_height: u32,
) -> HashSet<(u32, u32)> {
    let mut legal_paths = HashSet::new();
    if current_height == 9 {
        legal_paths.insert(start_position);
        return legal_paths;
    }

    for direction in Direction::DIRECTIONS {
        let next_position = get_height(map, start_position, direction);

        if let Some(next_position) = next_position {
            if map[next_position.0 as usize][next_position.1 as usize] == current_height + 1 {
                legal_paths.extend(identify_legal_paths_part_1(
                    map,
                    next_position,
                    current_height + 1,
                ));
            }
        }
    }

    legal_paths
}

fn identify_legal_paths_part_2(
    map: &Vec<Vec<u32>>,
    start_position: (u32, u32),
    current_height: u32,
) -> u32 {
    if current_height == 9 {
        return 1;
    }
    let mut legal_paths = 0;
    for direction in Direction::DIRECTIONS {
        let next_position = get_height(map, start_position, direction);

        if let Some(next_position) = next_position {
            if map[next_position.0 as usize][next_position.1 as usize] == current_height + 1 {
                legal_paths += identify_legal_paths_part_2(map, next_position, current_height + 1);
            }
        }
    }

    legal_paths
}

fn get_height(
    map: &Vec<Vec<u32>>,
    position: (u32, u32),
    direction: Direction,
) -> Option<(u32, u32)> {
    match direction {
        Direction::Up => {
            if position.0 == 0 {
                None
            } else {
                Some((position.0 - 1, position.1))
            }
        }
        Direction::Down => {
            if position.0 == map.len() as u32 - 1 {
                None
            } else {
                Some((position.0 + 1, position.1))
            }
        }
        Direction::Left => {
            if position.1 == 0 {
                None
            } else {
                Some((position.0, position.1 - 1))
            }
        }
        Direction::Right => {
            if position.1 == map[0].len() as u32 - 1 {
                None
            } else {
                Some((position.0, position.1 + 1))
            }
        }
    }
}
