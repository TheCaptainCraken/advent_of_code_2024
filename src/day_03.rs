use std::{fs, io::Result};

const INPUT: &str = "data/day_03/input.txt";

fn main() -> Result<()> {
    let input = fs::read_to_string(INPUT)?;

    let mut iter = input.chars().peekable();

    let mut result1 = 0;
    let mut result2 = 0;
    let mut enabled = true;

    while let Some(c) = iter.next() {
        if c == 'd' {
            if let Some('o') = iter.peek() {
                _ = iter.next();
                if let Some('(') = iter.peek() {
                    _ = iter.next();
                    if let Some(')') = iter.peek() {
                        _ = iter.next();
                        enabled = true;
                    }
                } else if let Some('n') = iter.peek() {
                    _ = iter.next();
                    if let Some('\'') = iter.peek() {
                        _ = iter.next();
                        if let Some('t') = iter.peek() {
                            _ = iter.next();
                            if let Some('(') = iter.peek() {
                                _ = iter.next();
                                if let Some(')') = iter.peek() {
                                    _ = iter.next();
                                    enabled = false;
                                }
                            }
                        }
                    }
                }
            }
        }
        if c == 'm' {
            if let Some('u') = iter.peek() {
                _ = iter.next();
                if let Some('l') = iter.peek() {
                    _ = iter.next();
                    if let Some('(') = iter.peek() {
                        _ = iter.next();
                        let mut first_number = String::new();
                        while let Some(c) = iter.peek() {
                            if c.is_digit(10) {
                                first_number.push(c.clone());
                                _ = iter.next();
                            } else {
                                break;
                            }
                        }
                        if let Some(',') = iter.peek() {
                            _ = iter.next();
                            let mut second_number = String::new();
                            while let Some(c) = iter.peek() {
                                if c.is_digit(10) {
                                    second_number.push(c.clone());
                                    _ = iter.next();
                                } else {
                                    break;
                                }
                            }
                            if let Some(')') = iter.peek() {
                                _ = iter.next();
                                let result = first_number.parse::<i32>().unwrap()
                                    * second_number.parse::<i32>().unwrap();
                                if enabled {
                                    result2 += result;
                                }

                                result1 += result;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("First result is {}", result1);
    println!("Second result is {}", result2);

    Ok(())
}
