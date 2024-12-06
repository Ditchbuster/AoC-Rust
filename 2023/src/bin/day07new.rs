#[allow(unused_imports)]
use std::{
    cmp::min,
    collections::HashMap,
    env,
    fs::{self},
    ops::Range,
    str::Lines,
    usize,
};

fn main() {
    let day = 7;
    let exm = true;

    println!("Hello, world! Day {}", day);
    let _args: Vec<String> = env::args().collect();
    //dbg!(args);

    let contents = fs::read_to_string(format!(
        "data\\{}\\day{}.txt",
        if exm { "example" } else { "input" },
        day
    ))
    .unwrap();
    let hands: Vec<Hand> = contents
        .lines()
        .map(|s| {
            let p: Vec<&str> = s.split_whitespace().collect();
            Hand::new(p[0].chars().collect(), p[1].parse().unwrap())
        })
        .collect();
    dbg!(hands);
    println!("Part A:{}", 0);

    println!("Part B:{}", 0);
}

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    counts: HashMap<char, u32>,
    bid: u32,
    hand_type: HandType,
}
impl Hand {
    fn new(cards: Vec<char>, bid: u32) -> Hand {
        let mut hand = Hand {
            cards,
            counts: HashMap::new(),
            bid,
            hand_type: HandType::HIGH,
        };
        for c in &hand.cards {
            hand.counts
                .entry(*c)
                .and_modify(|e| {
                    *e += 1;
                })
                .or_insert(1);
        }
        hand
    }
    fn calc_type(&self){
        let nums = 
    }
}
#[derive(Debug)]
enum HandType {
    HIGH,
    PAIR,
    TWOPAIR,
    THREEKIND,
    FULLHOUSE,
    FOURKIND,
    FIVEKIND,
}
