use std::env;

mod day01;
mod utils;
fn main() {
    let day = env::args().skip(1).next().expect("missing argument");
    match day.as_str() {
        "01" => {
            println!("1: {}", day01::solve1());
            println!("2: {}", day01::solve2());
        }
        _ => {
            panic!("wrong day {day}")
        }
    }
}
