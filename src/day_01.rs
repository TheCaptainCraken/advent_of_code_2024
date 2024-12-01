use std::{fs, io::Result};

const INPUT: &str = "data/day_01/input.txt";

fn main() -> Result<()> {
    let file = fs::read_to_string(INPUT)?;
    let mut left_numbers = Vec::new();
    let mut right_numbers = Vec::new();

    for line in file.lines() {
        let mut left_number = String::new();
        let mut right_number = String::new();
        let mut first_or_second = true;
        for c in line.chars() {
            if c == ' ' {
                first_or_second = false;
            } else if first_or_second {
                left_number.push(c);
            } else {
                right_number.push(c);
            }
        }

        left_numbers.push(left_number.parse::<i32>().unwrap());
        right_numbers.push(right_number.parse::<i32>().unwrap());
    }

    left_numbers.sort();
    right_numbers.sort();

    let result: i32 = left_numbers
        .iter()
        .zip(right_numbers.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();

    println!("First result: {}", result);

    let result2: usize = left_numbers
        .iter()
        .map(|l| right_numbers.iter().filter(|r| l == *r).count() * usize::try_from(*l).unwrap())
        .sum();

    println!("Second result: {}", result2);
    Ok(())
}
