use std::usize;
#[allow(unused_imports)]
use std::{
    cmp::max,
    collections::HashMap,
    env,
    fs::{self},
};

fn main() {
    let day = 4;
    let exm = false;

    println!("Hello, world! Day {}", day);
    let _args: Vec<String> = env::args().collect();
    //dbg!(args);

    let contents = fs::read_to_string(format!(
        "data\\{}\\day{}.txt",
        if exm { "example" } else { "input" },
        day
    ))
    .unwrap();

    let mut total: usize = 0;
    let mut cards: HashMap<usize, usize> = HashMap::new();
    for line in contents.lines() {
        let parts: Vec<&str> = line.split(&['|', ':'][..]).collect();
        let card_num: usize = parts[0]
            .split_whitespace()
            .skip(1)
            .next()
            .unwrap()
            .parse()
            .unwrap();
        *cards.entry(card_num).or_insert(0) += 1;
        //dbg!(&card_num, &cards[&card_num]);
        let win: Vec<usize> = parts[1]
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        let nums: Vec<usize> = parts[2]
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        let mut wins = 0;
        for n in win {
            if nums.contains(&n) {
                wins += 1;
                *cards.entry(card_num + &wins).or_insert(0) += cards[&card_num];
            }
        }
        if wins > 0 {
            total += 2_usize.pow((wins - 1).try_into().unwrap());
        }
    }
    println!("Part A:{}", total);
    //dbg!(&cards);
    total = cards.values().fold(0, |acc, x| acc + x);
    println!("Part B:{}", total);
}
