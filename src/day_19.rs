use std::collections::HashMap;

const INPUT: &str = include_str!("../data/day_19/input.txt");

fn main() {
    let lines = INPUT.lines().collect::<Vec<&str>>();

    let available_towels = lines[0]
        .split(", ")
        .map(|towel| towel.to_string())
        .collect::<Vec<String>>();

    let patterns = lines[2..]
        .iter()
        .map(|pattern| pattern.to_string())
        .collect::<Vec<String>>();

    let mut cache_1 = HashMap::new();
    let mut cache_2 = HashMap::new();

    let result_1 = patterns
        .iter()
        .filter(|pattern| is_valid_pattern(pattern, &available_towels, &mut cache_1))
        .count();

    let result_2 = patterns.iter().fold(0, |acc, pattern| {
        acc + count_valid_patterns(pattern, &available_towels, &mut cache_2)
    });

    println!("First part: {}", result_1);
    println!("Second part: {}", result_2);
}

fn is_valid_pattern(
    pattern: &String,
    available_towels: &Vec<String>,
    cache: &mut HashMap<String, bool>,
) -> bool {
    if let Some(&result) = cache.get(pattern) {
        return result;
    }

    if pattern.is_empty() {
        return true;
    }

    let mut valid_patterns = Vec::new();
    for towel in available_towels {
        if pattern.starts_with(towel) {
            let new_pattern = pattern[towel.len()..].to_string();
            valid_patterns.push(is_valid_pattern(&new_pattern, available_towels, cache));
        }
    }

    let result = valid_patterns.iter().any(|&x| x);

    cache.insert(pattern.to_string(), result);

    result
}

fn count_valid_patterns(
    pattern: &String,
    available_towels: &Vec<String>,
    cache: &mut HashMap<String, u128>,
) -> u128 {
    if let Some(&result) = cache.get(pattern) {
        return result;
    }

    if pattern.is_empty() {
        return 1;
    }

    let mut valid_patterns = 0;
    for towel in available_towels {
        if pattern.starts_with(towel) {
            let new_pattern = pattern[towel.len()..].to_string();
            valid_patterns += count_valid_patterns(&new_pattern, available_towels, cache);
        }
    }

    cache.insert(pattern.to_string(), valid_patterns);

    valid_patterns
}
