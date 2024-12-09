const INPUT: &str = include_str!("../data/day_09/input.txt");

fn main() {
    let input = INPUT.chars().map(|c| c.to_digit(10).unwrap());

    let mut disk = Vec::new();

    input.enumerate().for_each(|(index, number)| {
        if index % 2 != 0 {
            let mut add = Vec::new();

            for _ in 0..number {
                add.push(-1);
            }

            disk.extend(add);
        } else {
            let mut add = Vec::new();

            for _ in 0..number {
                add.push(index as i32 / 2);
            }

            disk.extend(add);
        }
    });

    let blank_indexes = disk
        .iter()
        .enumerate()
        .filter(|(_, &x)| x == -1)
        .map(|(i, _)| i);

    let mut clean_disk = disk.clone();

    for blank_index in blank_indexes {
        while clean_disk.last().unwrap() == &-1 {
            clean_disk.pop();
        }

        if clean_disk.len() <= blank_index {
            break;
        }

        clean_disk[blank_index] = clean_disk.pop().unwrap();
    }

    let result: i128 = clean_disk
        .iter()
        .enumerate()
        .fold(0, |sum, (index, element)| {
            sum + index as i128 * (*element as i128)
        });

    println!("Result first part: {}", result);
}
