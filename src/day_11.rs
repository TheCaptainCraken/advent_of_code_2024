use std::collections::HashMap;

const INPUT: &str = include_str!("../data/day_11/input.txt");

fn main() {
    let mut parsed_input_first_part = INPUT
        .split(' ')
        .map(|n| n.parse::<u128>().unwrap())
        .collect::<Vec<u128>>();

    for _ in 0..25 {
        let mut result = Vec::new();

        for number in parsed_input_first_part {
            if let Some(n) = first_rule(number) {
                result.push(n);
            } else if let Some(n) = second_rule(number) {
                result.push(n.0);
                result.push(n.1);
            } else {
                result.push(third_rule(number));
            }
        }

        parsed_input_first_part = result.clone();
    }

    println!("Part one: {}", parsed_input_first_part.len());

    let mut cached_inputs = HashMap::new();

    let parsed_input_second_part = INPUT
        .split(' ')
        .map(|n| n.parse::<u128>().unwrap())
        .collect::<Vec<u128>>();

    let answer_second_part = parsed_input_second_part
        .iter()
        .map(|n| recursive_count_baby(*n, 75, &mut cached_inputs))
        .sum::<u128>();

    println!("Part two: {}", answer_second_part);
}

fn recursive_count_baby(input: u128, depth: u8, cache: &mut HashMap<(u128, u8), u128>) -> u128 {
    if depth == 0 {
        1
    } else {
        if let Some(cached) = cache.get(&(input, depth)) {
            return *cached;
        } else if let Some(n) = first_rule(input) {
            let result = recursive_count_baby(n, depth - 1, cache);
            cache.insert((input, depth), result);
            result
        } else if let Some(ns) = second_rule(input) {
            let result = recursive_count_baby(ns.0, depth - 1, cache)
                + recursive_count_baby(ns.1, depth - 1, cache);

            cache.insert((input, depth), result);
            result
        } else {
            let result = recursive_count_baby(third_rule(input), depth - 1, cache);
            cache.insert((input, depth), result);
            result
        }
    }
}

fn first_rule(number: u128) -> Option<u128> {
    if number == 0 {
        Some(1)
    } else {
        None
    }
}

fn second_rule(number: u128) -> Option<(u128, u128)> {
    let characters = number.to_string();

    if characters.len() % 2 == 0 {
        let first_number = characters[0..characters.len() / 2].parse().unwrap();
        let second_number = characters[characters.len() / 2..].parse().unwrap();

        Some((first_number, second_number))
    } else {
        None
    }
}

fn third_rule(number: u128) -> u128 {
    number * 2024
}
