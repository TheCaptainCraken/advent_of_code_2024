use itertools::Itertools;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../data/day_08/input.txt");

fn main() {
    let number_of_rows = INPUT.lines().count();
    let number_of_columns = INPUT.lines().next().unwrap().chars().count();
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for (row, line) in INPUT.lines().enumerate() {
        for (column, letter) in line.chars().enumerate() {
            if letter == '.' {
                continue;
            }
            antennas
                .entry(letter)
                .or_insert_with(Vec::new)
                .push((row as i32, column as i32));
        }
    }
    let mut answer_part_one = HashSet::new();
    let mut answer_part_two = HashSet::new();

    antennas.iter().for_each(|(_, positions)| {
        positions.iter().combinations(2).for_each(|combo| {
            let (r1, c1) = (combo[0].0, combo[0].1);
            let (r2, c2) = (combo[1].0, combo[1].1);

            answer_part_one.insert((2 * r1 - r2, 2 * c1 - c2));
            answer_part_one.insert((2 * r2 - r1, 2 * c2 - c1));

            let (dr, dc) = (r2 - r1, c2 - c1);

            let mut row = r1;
            let mut col = c1;

            while row >= 0
                && row < number_of_rows as i32
                && col >= 0
                && col < number_of_columns as i32
            {
                answer_part_two.insert((row, col));
                row += dr;
                col += dc;
            }

            let mut row = r1;
            let mut col = c1;

            while row >= 0
                && row < number_of_rows as i32
                && col >= 0
                && col < number_of_columns as i32
            {
                answer_part_two.insert((row, col));
                row -= dr;
                col -= dc;
            }
        });
    });

    answer_part_one = answer_part_one
        .iter()
        .cloned()
        .filter(|(r, c)| {
            r >= &0 && r < &(number_of_rows as i32) && c >= &0 && c < &(number_of_columns as i32)
        })
        .collect();

    println!("Part one: {}", answer_part_one.len());
    println!("Part two: {}", answer_part_two.len());
}
