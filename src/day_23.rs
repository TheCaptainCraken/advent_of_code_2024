use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../data/day_23/input.txt");

fn main() {
    let mut map: HashMap<String, HashSet<String>> = HashMap::new();

    INPUT.lines().for_each(|line| {
        let x = line[0..2].to_string();
        let y = line[3..5].to_string();

        if !map.contains_key(&x) {
            map.insert(x.clone(), HashSet::new());
        }
        if !map.contains_key(&y) {
            map.insert(y.clone(), HashSet::new());
        }

        map.get_mut(&x).unwrap().insert(y.clone());
        map.get_mut(&y).unwrap().insert(x.clone());
    });

    let mut result = HashSet::new();
    for (x, x_connections) in map.iter() {
        for y in x_connections.iter() {
            for z in map.get(y).unwrap().iter() {
                if x != z && map.get(z).unwrap().contains(x) {
                    // sort x,y,z to avoid duplicates
                    let mut use_arr = [x.clone(), y.clone(), z.clone()];
                    use_arr.sort();
                    let (x, y, z) = (use_arr[0].clone(), use_arr[1].clone(), use_arr[2].clone());

                    result.insert((x, y, z));
                }
            }
        }
    }

    let first_part_result = result
        .iter()
        .filter(|(x, y, z)| {
            x.chars().next().unwrap() == 't'
                || y.chars().next().unwrap() == 't'
                || z.chars().next().unwrap() == 't'
        })
        .count();

    println!("First part: {}", first_part_result);
}
