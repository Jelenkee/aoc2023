use crate::utils::{line_to_numbers, parse_to_lines};
use std::fmt::Display;

pub fn solve1() -> impl Display {
    let lines = parse_to_lines("06.txt");
    let times = line_to_numbers(lines[0].split(':').nth(1).unwrap());
    let distances = line_to_numbers(lines[1].split(':').nth(1).unwrap());
    let mut product = 1;
    for i in 0..times.len() {
        let time = times[i];
        let distance = distances[i];
        let won_races = get_distances(time.into())
            .into_iter()
            .filter(|d| *d > distance.into())
            .count();
        product *= won_races;
    }
    product
}

pub fn solve2() -> impl Display {
    let lines = parse_to_lines("06.txt");
    let time = lines[0]
        .split(':')
        .nth(1)
        .unwrap()
        .replace(|c: char| c.is_ascii_whitespace(), "")
        .parse::<u64>()
        .unwrap();
    let distance = lines[1]
        .split(':')
        .nth(1)
        .unwrap()
        .replace(|c: char| c.is_ascii_whitespace(), "")
        .parse::<u64>()
        .unwrap();
    get_distances(time)
        .into_iter()
        .filter(|d| *d > distance)
        .count()
}

fn get_distances(max_time: u64) -> Vec<u64> {
    (1..max_time).map(|t| get_distance(t, max_time)).collect()
}

fn get_distance(charging_ms: u64, max_time: u64) -> u64 {
    if charging_ms == 0 {
        return 0;
    }
    let remaining = max_time - charging_ms;
    remaining * charging_ms
}
