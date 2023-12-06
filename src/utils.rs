use grid::{grid, Grid};
use std::fs;

pub fn get_input(filename: &str) -> String {
    fs::read_to_string(format!("./input/{filename}"))
        .unwrap()
        .trim()
        .to_string()
}
pub fn parse_to_lines(filename: &str) -> Vec<String> {
    get_input(filename)
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

pub fn to_grid(lines: Vec<String>) -> Grid<char> {
    let mut grid = grid![];
    for (row, line) in lines.into_iter().enumerate() {
        grid.insert_row(row, line.chars().collect());
    }
    grid
}

pub fn line_to_numbers(s: &str) -> Vec<u32> {
    s.split(' ')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}
