use std::env;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod utils;
fn main() {
    let day = env::args().nth(1).expect("missing argument");
    match day.as_str() {
        "01" => {
            println!("1: {}", day01::solve1());
            println!("2: {}", day01::solve2());
        }
        "02" => {
            println!("1: {}", day02::solve1());
            println!("2: {}", day02::solve2());
        }
        "03" => {
            println!("1: {}", day03::solve1());
            println!("2: {}", day03::solve2());
        }
        "04" => {
            println!("1: {}", day04::solve1());
            println!("2: {}", day04::solve2());
        }
        "05" => {
            println!("1: {}", day05::solve1());
            println!("2: {}", day05::solve2());
        }
        "06" => {
            println!("1: {}", day06::solve1());
            println!("2: {}", day06::solve2());
        }
        _ => {
            panic!("wrong day {day}")
        }
    }
}
