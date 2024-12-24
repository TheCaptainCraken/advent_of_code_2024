const INPUT: &str = include_str!("../data/day_22/input.txt");
const PRUNE_FACTOR: u128 = 16777216;

fn main() {
    let initial_secret_numbers = INPUT
        .lines()
        .map(|line| line.parse::<u128>().unwrap())
        .collect::<Vec<u128>>();

    let result = initial_secret_numbers
        .iter()
        .map(|&secret_number| {
            let mut secret_number = secret_number;
            for _ in 0..2000 {
                secret_number = next_secret_number(secret_number);
            }

            secret_number
        })
        .collect::<Vec<u128>>();

    println!("First part: {}", result.iter().sum::<u128>());
}

fn next_secret_number(secret_number: u128) -> u128 {
    let mut secret_number = secret_number;

    // first operation
    let result = secret_number << 6; // secret_number * 64
    secret_number = result ^ secret_number;
    secret_number = secret_number % PRUNE_FACTOR;

    // second operation
    let result = secret_number >> 5; // secret_number / 32
    secret_number = result ^ secret_number;
    secret_number = secret_number % PRUNE_FACTOR;

    // third operation
    let result = secret_number << 11; // secret_number * 2048
    secret_number = result ^ secret_number;
    secret_number = secret_number % PRUNE_FACTOR;

    secret_number
}
