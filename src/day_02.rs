use std::{fs, io::Result};

const INPUT: &str = "data/day_02/input.txt";

fn main() -> Result<()> {
    let file = fs::read_to_string(INPUT)?;

    let reports = file
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let good_reports = reports
        .iter()
        .filter(|report| {
            let is_safe = (report.is_sorted() || report.is_sorted_by(|a, b| b <= a))
                && report.windows(2).all(|numbers| {
                    (numbers[0] - numbers[1]).abs() <= 3 && (numbers[0] - numbers[1]).abs() >= 1
                });
            is_safe
        })
        .count();

    println!("Good reports: {}", good_reports);

    // Brute force solution for part 2 :/

    let mut valid_reports = 0;
    for report in reports {
        let valid = (report.is_sorted() || report.is_sorted_by(|a, b| b <= a))
            && report.windows(2).all(|numbers| {
                (numbers[0] - numbers[1]).abs() <= 3 && (numbers[0] - numbers[1]).abs() >= 1
            });

        if valid {
            valid_reports += 1;
        } else {
            for i in 0..report.len() {
                let mut new_report = report.clone();
                new_report.remove(i);
                let valid = (new_report.is_sorted() || new_report.is_sorted_by(|a, b| b <= a))
                    && new_report.windows(2).all(|numbers| {
                        (numbers[0] - numbers[1]).abs() <= 3 && (numbers[0] - numbers[1]).abs() >= 1
                    });
                if valid {
                    valid_reports += 1;
                    break;
                }
            }
        }
    }

    println!("Valid reports with problem dampener: {}", valid_reports);
    Ok(())
}
