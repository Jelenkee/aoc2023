use crate::utils::parse_to_lines;
use regex::Regex;
use std::fmt::Display;

pub fn solve1() -> impl Display {
    let lines = parse_to_lines("test.txt");
    let mut sum = 0usize;
    for (index, line) in lines.into_iter().enumerate() {
        let ex = extract_max_cubes(&line);
        if ex.0 <= 12 && ex.1 <= 13 && ex.2 <= 14 {
            sum += index + 1;
        }
    }

    sum
}

pub fn solve2() -> impl Display {
    let lines = parse_to_lines("test.txt");
    let mut sum = 0u32;
    for line in lines {
        let ex = extract_max_cubes(&line);
        sum += ex.0 * ex.1 * ex.2;
    }

    sum
}

fn extract_max_cubes(s: &str) -> (u32, u32, u32) {
    let r = Regex::new(r"(?<num>\d+) (?<color>\w+)").unwrap();
    let v = r
        .captures_iter(s)
        .map(|cap| {
            let (_, [num, color]) = cap.extract();
            (num.parse::<u32>().unwrap(), color.to_string())
        })
        .collect::<Vec<_>>();

    let extractor = |color: &str| {
        *v.iter()
            .filter(|(_, s)| s == color)
            .map(|(n, _)| n)
            .max()
            .unwrap()
    };
    (extractor("red"), extractor("green"), extractor("blue"))
}
