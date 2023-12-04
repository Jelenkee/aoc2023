use std::collections::HashMap;
use std::fmt::Display;

use crate::utils::parse_to_lines;

pub fn solve1() -> impl Display {
    let lines = parse_to_lines("04.txt");
    let mut sum = 0u32;
    for line in lines {
        let (left, right) = parse_line(&line);
        let count = count_matches(&left, &right);
        if count > 0 {
            sum += 2u32.pow(count - 1);
        }
    }
    sum
}

pub fn solve2() -> impl Display {
    let lines = parse_to_lines("04.txt");
    let mut iteration_map = HashMap::new();
    let mut card_map = HashMap::new();
    for (index, line) in lines.into_iter().enumerate() {
        let card_id = index + 1;
        iteration_map.insert(card_id, 1u32);
        let (left, right) = parse_line(&line);
        let count = count_matches(&left, &right);
        card_map.insert(card_id, count);
    }
    let mut ids = iteration_map.keys().copied().collect::<Vec<_>>();
    ids.sort();
    for id in ids.iter() {
        let count = *card_map.get(id).unwrap() as usize;
        let iterations = *iteration_map.get(id).unwrap() as usize;
        for n in (id + 1)..(id + 1 + (count)) {
            for _ in 0..iterations {
                iteration_map.entry(n).and_modify(|v| (*v) += 1);
            }
        }
    }
    iteration_map.into_values().sum::<u32>()
}

fn parse_line(s: &str) -> (Vec<u8>, Vec<u8>) {
    let parts = s.split([':', '|']).collect::<Vec<_>>();
    (line_to_numbers(parts[1]), line_to_numbers(parts[2]))
}

fn line_to_numbers(s: &str) -> Vec<u8> {
    s.split(' ')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}

fn count_matches(left: &Vec<u8>, right: &[u8]) -> u32 {
    let mut count = 0;
    for n in left {
        if right.contains(n) {
            count += 1;
        }
    }
    count
}
