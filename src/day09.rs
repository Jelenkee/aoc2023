use crate::utils::parse_to_lines;
use std::fmt::Display;

pub fn solve1() -> impl Display {
    let lines = parse_to_lines("09.txt");

    lines
        .into_iter()
        .map(|l| line_to_numbers(&l))
        .map(|mut v| predict_next_value(&v))
        .sum::<i64>()
}

pub fn solve2() -> impl Display {
    let lines = parse_to_lines("09.txt");

    lines
        .into_iter()
        .map(|l| line_to_numbers(&l))
        .map(|mut v| predict_previous_value(&v))
        .sum::<i64>()
}

fn predict_next_value(vec: &Vec<i64>) -> i64 {
    let mut stored_vecs: Vec<Vec<i64>> = vec![];
    stored_vecs.push(vec.clone());
    loop {
        let last_vec = stored_vecs.last().unwrap().clone();
        let new_vec = difference_vec(&last_vec);
        *stored_vecs.last_mut().unwrap() = last_vec;
        stored_vecs.push(new_vec.clone());
        if new_vec.iter().all(|n| *n == 0) {
            break;
        }
    }
    stored_vecs.last_mut().unwrap().push(0);
    for i in (1..stored_vecs.len()).rev() {
        let a1 = *stored_vecs[i].last().unwrap();
        let a2 = *stored_vecs[i - 1].last().unwrap();
        stored_vecs[i - 1].push(a1 + a2);
    }

    *stored_vecs[0].last().unwrap()
}

fn predict_previous_value(vec: &Vec<i64>) -> i64 {
    let mut stored_vecs: Vec<Vec<i64>> = vec![];
    stored_vecs.push(vec.clone());
    loop {
        let last_vec = stored_vecs.last().unwrap().clone();
        let new_vec = difference_vec(&last_vec);
        *stored_vecs.last_mut().unwrap() = last_vec;
        stored_vecs.push(new_vec.clone());
        if new_vec.iter().all(|n| *n == 0) {
            break;
        }
    }

    stored_vecs.last_mut().unwrap().insert(0, 0);
    for i in (1..stored_vecs.len()).rev() {
        let a1 = *stored_vecs[i].first().unwrap();
        let a2 = *stored_vecs[i - 1].first().unwrap();
        stored_vecs[i - 1].insert(0, a2 - a1);
    }

    *stored_vecs[0].first().unwrap()
}

fn difference_vec(vec: &Vec<i64>) -> Vec<i64> {
    if vec.is_empty() {
        panic!("vec is empty");
    }
    let mut new_vec = vec![];

    for i in 0..(vec.len() - 1) {
        let first = vec[i];
        let second = vec[i + 1];
        new_vec.push(second - first);
    }
    new_vec
}

fn line_to_numbers(s: &str) -> Vec<i64> {
    s.split(' ')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}
