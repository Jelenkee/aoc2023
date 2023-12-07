use crate::utils::parse_to_lines;
use std::{collections::HashMap, fmt::Display};

const LABELS: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

pub fn solve1() -> impl Display {
    return 0;
    let lines = parse_to_lines("07.txt");
    let mut hand_pairs = vec![];
    for line in lines {
        let pair = line.split_ascii_whitespace().collect::<Vec<_>>();
        let hand = Hand::from(pair[0]);
        let bid = pair[1].parse::<usize>().unwrap();
        hand_pairs.push((hand, bid));
    }
    hand_pairs.sort_by(|a, b| a.0.cmp(&b.0));
    let mut sum = 0;
    for (index, (_, bid)) in hand_pairs.into_iter().enumerate() {
        sum += (index + 1) * bid
    }
    sum
}

pub fn solve2() -> impl Display {
    let lines = parse_to_lines("07.txt");
    let mut hand_pairs = vec![];
    for line in lines {
        let pair = line.split_ascii_whitespace().collect::<Vec<_>>();
        let hand = Hand::from(pair[0]);
        let bid = pair[1].parse::<usize>().unwrap();
        hand_pairs.push((hand, bid));
    }
    hand_pairs.sort_by(|a, b| a.0.cmp(&b.0));
    let mut sum = 0;
    for (index, (_, bid)) in hand_pairs.into_iter().enumerate() {
        sum += (index + 1) * bid
    }
    sum
}

fn get_combo(hand: &Hand) -> Combo {
    let mut map = HashMap::new();
    for c in hand.0.iter() {
        let value = map.entry(c.0).or_insert(0);
        (*value) += 1;
    }
    let values = map.into_values().collect::<Vec<_>>();
    if values.contains(&5) {
        Combo::Quintuple
    } else if values.contains(&4) {
        Combo::Quadruple
    } else if values.contains(&3) && values.contains(&2) {
        Combo::FullHouse
    } else if values.contains(&3) {
        Combo::Triple
    } else if values.iter().filter(|i| **i == 2).count() == 2 {
        Combo::Pair2
    } else if values.contains(&2) {
        Combo::Pair
    } else {
        Combo::High
    }
}

fn get_combo2(hand: &Hand) -> Combo {
    if !hand.to_string().contains('J') {
        return get_combo(hand);
    }
    let mut map = HashMap::new();
    for c in hand.0.iter() {
        if c.0 == 'J' {
            continue;
        }
        let value = map.entry(c.0).or_insert(0);
        (*value) += 1;
    }
    let best_char = {
        let mut entries = map.into_iter().collect::<Vec<_>>();
        entries.sort_by(|a, b| Label(b.0).cmp(&Label(a.0)));
        entries.sort_by(|a, b| b.1.cmp(&a.1));
        if entries.is_empty() {
            *LABELS.last().unwrap()
        } else {
            entries[0].0
        }
    };
    let transformed_hand: &Hand = &hand
        .to_string()
        .replace('J', &best_char.to_string())
        .as_str()
        .into();
    get_combo(transformed_hand)
}

#[derive(PartialEq, Eq, Debug)]
struct Label(char);
impl Ord for Label {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let pos1 = LABELS.iter().position(|l| *l == self.0).unwrap();
        let pos2 = LABELS.iter().position(|l| *l == other.0).unwrap();
        pos1.cmp(&pos2)
    }
}
impl PartialOrd for Label {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[derive(PartialEq, Eq, Debug)]
struct Hand([Label; 5]);
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        //let combo1 = get_combo(self);
        //let combo2 = get_combo(other);
        let combo1 = get_combo2(self);
        let combo2 = get_combo2(other);
        let ord = combo1.cmp(&combo2);
        if ord.is_ne() {
            return ord;
        }
        self.0.cmp(&other.0)
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        assert!(value.chars().count() == 5);
        Hand(
            value
                .chars()
                .map(Label)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        )
    }
}

impl ToString for Hand {
    fn to_string(&self) -> String {
        self.0.iter().map(|l| l.0).collect::<String>()
    }
}

#[derive(PartialEq, Eq, Debug, PartialOrd, Ord)]
enum Combo {
    High,
    Pair,
    Pair2,
    Triple,
    FullHouse,
    Quadruple,
    Quintuple,
}
