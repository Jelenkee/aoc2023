use std::collections::HashMap;
use std::fmt::Display;

use crate::utils::parse_to_lines;

const KEYS: [&str; 7] = [
    "seed",
    "soil",
    "fertilizer",
    "water",
    "light",
    "temperature",
    "humidity",
];
pub fn solve1() -> impl Display {
    let mut lines = parse_to_lines("05.txt");
    let seed_line = lines.remove(0);
    let seeds = split_line(seed_line.split(':').nth(1).unwrap())
        .into_iter()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let mapmap = get_map(&lines);

    let mut locations = vec![];
    for seed in seeds {
        let mut first = seed;
        for key in KEYS {
            let map = mapmap.get(key).unwrap();
            first = map.get(&first);
        }
        locations.push(first);
    }
    locations.into_iter().min().unwrap()
}

pub fn solve2() -> impl Display {
    let mut lines = parse_to_lines("05.txt");
    let seed_line = lines.remove(0);
    let seeds = split_line(seed_line.split(':').nth(1).unwrap())
        .into_iter()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let mapmap = get_map(&lines);
    let mut min = u64::MAX;
    for seed in seeds.chunks(2) {
        let start = seed[0];
        let steps = seed[1];
        for i in 0..steps {
            let seeed = start + i;
            let mut first = seeed;
            for key in KEYS {
                let map = mapmap.get(key).unwrap();
                first = map.get(&first);
            }
            min = min.min(first);
        }
    }
    min
}

fn split_line(s: &str) -> Vec<&str> {
    s.split(' ')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect()
}

fn get_map(lines: &Vec<String>) -> HashMap<&str, PseudoMap> {
    let mut mapmap = HashMap::new();
    let mut index = 0;
    let mut keys = vec![];
    let mut values = vec![];
    let mut ranges = vec![];
    for line in lines {
        if line.contains("map:") {
            if !keys.is_empty() {
                let map = PseudoMap::new(keys, values, ranges);
                mapmap.insert(KEYS[index - 1], map);
                keys = vec![];
                values = vec![];
                ranges = vec![];
            }
            index += 1;
            continue;
        }
        let numbers = split_line(line)
            .into_iter()
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        keys.push(numbers[1]);
        values.push(numbers[0]);
        ranges.push(numbers[2]);
    }
    let map = PseudoMap::new(keys, values, ranges);
    mapmap.insert(KEYS[index - 1], map);
    mapmap
}

#[derive(Debug)]
struct PseudoMap {
    keys: Vec<u64>,
    values: Vec<u64>,
    ranges: Vec<u64>,
}
impl PseudoMap {
    fn new(keys: Vec<u64>, values: Vec<u64>, ranges: Vec<u64>) -> Self {
        assert_eq!(keys.len(), values.len());
        PseudoMap {
            keys,
            values,
            ranges,
        }
    }
    fn get(&self, key: &u64) -> u64 {
        for index in 0..self.keys.len() {
            let self_key = self.keys[index];
            let self_value = self.values[index];
            let self_range = self.ranges[index];
            if *key >= self_key && *key < self_key + self_range {
                let diff = key - self_key;
                return self_value + diff;
            }
        }
        *key
    }
}
