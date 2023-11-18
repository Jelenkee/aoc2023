use std::fmt::Display;
use std::fs;

pub fn get_input(filename: &str) -> String {
    fs::read_to_string(format!("./input/{filename}")).unwrap()
}
pub fn parse_to_lines(filename: &str) -> Vec<String> {
    get_input(filename)
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}
