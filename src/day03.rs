use crate::utils::{parse_to_lines, to_grid};
use grid::Grid;
use regex::Regex;
use std::fmt::Display;

type Pos = (usize, usize);
pub fn solve1() -> impl Display {
    let lines = parse_to_lines("03.txt");
    let grid = to_grid(lines);
    collect_numbers(&grid)
        .into_iter()
        .filter(|(_, (a, b))| is_number_surrounded(&grid, (*a, *b)).is_some())
        .map(|(num, _)| num)
        .sum::<u32>()
}

pub fn solve2() -> impl Display {
    let lines = parse_to_lines("03.txt");
    let grid = to_grid(lines);
    let numbers = collect_numbers(&grid);
    let gears = get_gear_positions(&grid);
    let mut sum = 0u32;
    for gear in gears {
        let mut nums = vec![];
        for (value, num_pos) in &numbers {
            let min_distance = distance_sq(&num_pos.0, &gear).min(distance_sq(&num_pos.1, &gear));
            if min_distance <= 2 {
                nums.push(value);
            }
        }
        if nums.len() == 2 {
            sum += nums[0] * nums[1];
        }
    }
    sum
}

fn collect_numbers(grid: &Grid<char>) -> Vec<(u32, (Pos, Pos))> {
    let regex = Regex::new(r"\d+").unwrap();
    let mut vec = vec![];
    for (index, row) in grid.iter_rows().enumerate() {
        let ss = row.clone().collect::<String>();
        for mat in regex.find_iter(&ss) {
            vec.push((
                mat.as_str().parse().unwrap(),
                ((mat.start(), index), (mat.end() - 1, index)),
            ));
        }
    }
    vec
}
fn is_number_surrounded(grid: &Grid<char>, range: (Pos, Pos)) -> Option<(char, Pos)> {
    for x in (range.0 .0.saturating_sub(1))..=(range.1 .0 + 1).min(grid.cols() - 1) {
        for y in (range.0 .1.saturating_sub(1))..=(range.1 .1 + 1).min(grid.rows() - 1) {
            let c = grid[(y, x)];
            if !c.is_ascii_digit() && c != '.' {
                return Some((c, (x, y)));
            }
        }
    }
    None
}

fn get_gear_positions(grid: &Grid<char>) -> Vec<Pos> {
    let regex = Regex::new(r"\*").unwrap();
    let mut vec = vec![];
    for (index, row) in grid.iter_rows().enumerate() {
        let ss = row.clone().collect::<String>();
        for mat in regex.find_iter(&ss) {
            vec.push((mat.start(), index));
        }
    }
    vec
}

fn distance_sq(a: &Pos, b: &Pos) -> i32 {
    (a.0 as i32 - b.0 as i32).pow(2) + (a.1 as i32 - b.1 as i32).pow(2)
}
