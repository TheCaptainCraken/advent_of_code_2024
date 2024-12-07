const INPUT: &str = "data/day_07/input.txt";

fn main() {
    let input = std::fs::read_to_string(INPUT).unwrap();

    let mut sum_first_part = 0;
    let mut sum_second_part = 0;

    for line in input.lines() {
        let split_line = line.split(':').collect::<Vec<&str>>();
        let result = split_line[0].parse::<u128>().unwrap();
        let operands = split_line[1]
            .split_whitespace()
            .map(|n| n.parse::<u128>().unwrap())
            .rev()
            .collect::<Vec<u128>>();

        if check_first_part(result, &operands) {
            sum_first_part += result;
        }

        if check_second_part(result, &operands) {
            sum_second_part += result;
        }
    }

    println!("First part: {}", sum_first_part);
    println!("Second part: {}", sum_second_part);
}

fn check_second_part(result: u128, operands: &Vec<u128>) -> bool {
    if operands.len() == 1 {
        return operands[0] == result;
    } else {
        let mul = if result % operands[0] == 0 {
            let new_result = result / operands[0];
            let new_operands = operands[1..].to_vec();
            check_second_part(new_result, &new_operands)
        } else {
            false
        };
        let add = if result > operands[0] {
            let new_result = result - operands[0];
            let new_operands = operands[1..].to_vec();
            check_second_part(new_result, &new_operands)
        } else {
            false
        };

        let concat = {
            let result_str = result.to_string();
            let operand_str = operands[0].to_string();

            if result_str.len() > operand_str.len() && result_str.ends_with(&operand_str) {
                let new_result_str = result_str[..result_str.len() - operand_str.len()]
                    .parse()
                    .unwrap();
                let new_operands = operands[1..].to_vec();
                check_second_part(new_result_str, &new_operands)
            } else {
                false
            }
        };

        return mul || add || concat;
    }
}

fn check_first_part(result: u128, operands: &Vec<u128>) -> bool {
    if operands.len() == 1 {
        return operands[0] == result;
    } else {
        let mul = if result % operands[0] == 0 {
            let new_result = result / operands[0];
            let new_operands = operands[1..].to_vec();
            check_first_part(new_result, &new_operands)
        } else {
            false
        };
        let add = if result > operands[0] {
            let new_result = result - operands[0];
            let new_operands = operands[1..].to_vec();
            check_first_part(new_result, &new_operands)
        } else {
            false
        };

        return mul || add;
    }
}
