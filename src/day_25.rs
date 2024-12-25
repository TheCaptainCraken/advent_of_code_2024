const INPUT: &str = include_str!("../data/day_25/input.txt");

type KeyOrKeyHole = [u8; 5];

fn main() {
    let mut keys: Vec<KeyOrKeyHole> = Vec::new();
    let mut key_holes: Vec<KeyOrKeyHole> = Vec::new();

    let lines = INPUT.lines().collect::<Vec<&str>>();

    let mut key_or_key_hole: KeyOrKeyHole = [0; 5];
    let mut is_key = false;
    for (i, thing) in lines.iter().enumerate() {
        if i % 8 == 0 {
            is_key = *thing == ".....";
        }

        if i % 8 == 7 {
            key_or_key_hole.iter_mut().for_each(|x| *x -= 1);
            if is_key {
                keys.push(key_or_key_hole);
            } else {
                key_holes.push(key_or_key_hole);
            }

            key_or_key_hole = [0; 5];
            continue;
        }

        thing.chars().enumerate().for_each(|(j, c)| {
            if c == '#' {
                key_or_key_hole[j] += 1;
            }
        });
    }

    let mut count = 0;

    for key_hole in key_holes.iter() {
        for key in keys.iter() {
            if does_fit(key, key_hole) {
                count += 1;
            }
        }
    }

    println!("Part 1: {}", count);
}

fn does_fit(key: &KeyOrKeyHole, key_hole: &KeyOrKeyHole) -> bool {
    key.iter().zip(key_hole.iter()).all(|(k, kh)| k + kh <= 5)
}
