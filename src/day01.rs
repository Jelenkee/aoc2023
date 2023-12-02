use crate::utils::parse_to_lines;
use std::fmt::Display;

const WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn solve1() -> impl Display {
    let lines = parse_to_lines("01.txt");

    lines
        .iter()
        .map(|line| get_numbers_from_string(line))
        .map(get_number_from_list)
        .sum::<u32>()
}

pub fn solve2() -> impl Display {
    let lines = parse_to_lines("01.txt");

    lines
        .iter()
        .map(|line| vec![find_first_number(line), find_last_number(line)])
        .map(get_number_from_list)
        .sum::<u32>()
}

fn get_numbers_from_string(s: &str) -> Vec<u32> {
    s.chars().filter_map(|c| c.to_digit(10)).collect()
}

fn get_number_from_list(vec: Vec<u32>) -> u32 {
    let first = *vec.first().unwrap();
    let last = *vec.last().unwrap();
    first * 10 + last
}
fn find_first_number(s: &str) -> u32 {
    let mut found_word = String::new();
    for c in s.chars() {
        if let Some(num) = c.to_digit(10) {
            return num;
        }
        found_word.push(c);
        for (index, word) in WORDS.into_iter().enumerate() {
            if found_word.contains(word) {
                return (index + 1) as u32;
            }
        }
    }
    unreachable!();
}

fn find_last_number(s: &str) -> u32 {
    let mut found_word = String::new();
    for c in s.chars().rev() {
        if let Some(num) = c.to_digit(10) {
            return num;
        }
        found_word.push(c);
        for (index, word) in WORDS.into_iter().enumerate() {
            if found_word.contains(&word.chars().rev().collect::<String>()) {
                return (index + 1) as u32;
            }
        }
    }
    unreachable!();
}
