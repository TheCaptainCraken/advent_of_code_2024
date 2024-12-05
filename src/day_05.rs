use std::{collections::HashMap, fs, io::Result};

const INPUT: &str = "data/day_05/input.txt";

fn main() -> Result<()> {
    let file = fs::read_to_string(INPUT)?;
    let lines = file.lines().collect::<Vec<&str>>();

    let mut is_first_part = true;
    let mut pages: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    for line in lines {
        if line == "" {
            is_first_part = false;
            continue;
        }

        if is_first_part {
            let numbers = line
                .split('|')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            if pages.contains_key(&numbers[1]) {
                pages.get_mut(&numbers[1]).unwrap().push(numbers[0]);
            } else {
                pages.insert(numbers[1], vec![numbers[0]]);
            }
        }

        if !is_first_part {
            let numbers = line
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            updates.push(numbers);
        }
    }

    let mut sum = 0;
    let mut things_to_order = Vec::new();
    for update in &updates {
        let mut check = Vec::new();
        let mut passed = true;

        for number in update {
            if check.contains(number) {
                passed = false;
                break;
            }

            if pages.contains_key(number) {
                check.append(pages[number].clone().as_mut());
            }
        }

        if passed {
            let index = update.len() / 2;
            sum += update[index];
        } else {
            things_to_order.push(update);
        }
    }

    // b|a
    // b < a
    let mut second_sum = 0;
    for thing in things_to_order {
        let mut thing = thing.clone();
        thing.sort_by(|a, b| {
            if pages.contains_key(a) {
                if pages[a].contains(b) {
                    return std::cmp::Ordering::Greater;
                }
            }

            if pages.contains_key(b) {
                if pages[b].contains(a) {
                    return std::cmp::Ordering::Less;
                }
            }

            return std::cmp::Ordering::Equal;
        });

        let index = thing.len() / 2;
        second_sum += thing[index];
    }

    println!("First part: {}", sum);
    println!("Second part: {}", second_sum);

    Ok(())
}
