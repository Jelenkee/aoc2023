use crate::utils::parse_to_lines;
use std::{collections::HashMap, fmt::Display, vec};

pub fn solve1() -> impl Display {
    let mut lines = parse_to_lines("08.txt");
    let instructions = lines.remove(0).chars().collect::<Vec<_>>();
    let map = create_map(&lines);
    let mut current_key = &"AAA".to_string();
    let mut counter = 0;
    for instruction in instructions.into_iter().cycle() {
        if current_key == "ZZZ" {
            break;
        }
        let pair = map.get(current_key).unwrap();
        current_key = if instruction == 'L' { &pair.0 } else { &pair.1 };
        counter += 1;
    }

    counter
}

pub fn solve2() -> impl Display {
    let mut lines = parse_to_lines("08.txt");
    let instructions = lines.remove(0).chars().collect::<Vec<_>>();
    let map = create_map(&lines);
    let current_keys = map.keys().filter(|s| s.ends_with('A')).collect::<Vec<_>>();
    let mut mins = vec![];

    for i in 0..current_keys.len() {
        let mut current_key = current_keys[i];
        let mut counter = 0;
        for instruction in instructions.iter().cycle() {
            if current_key.ends_with('Z') {
                break;
            }
            let pair = map.get(current_key).unwrap();
            current_key = if *instruction == 'L' {
                &pair.0
            } else {
                &pair.1
            };
            counter += 1;
        }
        mins.push(counter);
    }

    mins.into_iter().fold(1, lcm)
}

fn create_map(lines: &Vec<String>) -> HashMap<String, (String, String)> {
    let mut map = HashMap::new();
    for line in lines {
        let [key, val] = line.split('=').collect::<Vec<&str>>().try_into().unwrap();
        let fixed_val = val.replace(['(', ')'], "");
        let mut val_iter = fixed_val.split(',');
        map.insert(
            key.trim().to_string(),
            (
                val_iter.next().unwrap().trim().to_string(),
                val_iter.next().unwrap().trim().to_string(),
            ),
        );
    }
    map
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}
