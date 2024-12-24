use std::collections::HashMap;

use itertools::Itertools;

const INPUT: &str = include_str!("../data/day_24/input.txt");

#[derive(Debug)]
enum Gate {
    And,
    Or,
    Xor,
}

impl Gate {
    fn solve(&self, input_1: u8, input_2: u8) -> u8 {
        match self {
            Self::And => input_1 & input_2,
            Self::Or => input_1 | input_2,
            Self::Xor => input_1 ^ input_2,
        }
    }

    fn from_str(s: &str) -> Self {
        match s {
            "AND" => Self::And,
            "OR" => Self::Or,
            "XOR" => Self::Xor,
            _ => panic!("Invalid gate"),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    gate: Gate,
    input_1: String,
    input_2: String,
    output: String,
    solved: bool,
}

impl Instruction {
    fn new(gate: Gate, input_1: String, input_2: String, output: String) -> Self {
        Self {
            gate,
            input_1,
            input_2,
            output,
            solved: false,
        }
    }
}

fn main() {
    let mut variables: HashMap<String, Option<u8>> = HashMap::new();
    let mut instructions: Vec<Instruction> = Vec::new();

    let mut is_second_part = false;
    for line in INPUT.lines() {
        if line.is_empty() {
            is_second_part = true;
            continue;
        }

        if !is_second_part {
            let variable_name = line[0..3].to_string();
            let value = line[5..].parse::<u8>().unwrap();

            variables.insert(variable_name, Some(value));
        } else {
            let line = line.split(' ').collect::<Vec<&str>>();

            let operand1 = line[0].to_string();
            let operand2 = line[2].to_string();

            let gate = Gate::from_str(line[1]);

            let output = line[4].to_string();

            if !variables.contains_key(&operand1) {
                variables.insert(operand1.clone(), None);
            }

            if !variables.contains_key(&operand2) {
                variables.insert(operand2.clone(), None);
            }

            if !variables.contains_key(&output) {
                variables.insert(output.clone(), None);
            }
            instructions.push(Instruction::new(gate, operand1, operand2, output));
        }
    }

    let mut solved = false;

    while !solved {
        solved = true;

        for instruction in &mut instructions {
            if instruction.solved {
                continue;
            }

            let input_1 = match variables.get(&instruction.input_1).unwrap() {
                Some(value) => *value,
                None => {
                    solved = false;
                    continue;
                }
            };

            let input_2 = match variables.get(&instruction.input_2).unwrap() {
                Some(value) => *value,
                None => {
                    solved = false;
                    continue;
                }
            };

            let result = instruction.gate.solve(input_1, input_2);
            instruction.solved = true;

            variables
                .get_mut(&instruction.output)
                .unwrap()
                .replace(result);
        }
    }

    let result = variables
        .iter()
        .filter(|(var, _)| var[0..1].to_string() == "z")
        .sorted_by(|(var1, _), (var2, _)| var1.cmp(var2))
        .map(|(_, value)| value.unwrap())
        .enumerate()
        .fold(0, |acc, (index, value)| {
            acc + (value as usize) * 2usize.pow(index as u32)
        });

    println!("{:?}", result);
}
