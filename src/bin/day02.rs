#[allow(unused_imports)]
use std::{
    cmp::max,
    collections::HashMap,
    env,
    fs::{self},
};

fn main() {
    let day = 2;
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

    //let mut games: Vec<bool> = Vec::new();
    let mut total = 0;
    for (i, line) in contents.lines().enumerate() {
        if game_possible(line) {
            total += i + 1;
        }
    }
    println!("Part A:{}", total);
    let mut total = 0;
    for (_i, line) in contents.lines().enumerate() {
        total += partb(line);
    }
    println!("Part B:{}", total);
}
fn game_possible(line: &str) -> bool {
    let sets: Vec<&str> = line.split(&[':', ';'][..]).collect();
    //let sets: &str = line.split(":").collect::<Vec<&str>>()[1];
    for set in &sets[1..] {
        let parts: Vec<&str> = set.split(',').collect();
        for part in parts {
            let pair: Vec<&str> = part.split_whitespace().collect();
            match pair[1] {
                "blue" => {
                    if pair[0].parse::<i32>().unwrap() > 14 {
                        return false;
                    }
                }
                "red" => {
                    if pair[0].parse::<i32>().unwrap() > 12 {
                        return false;
                    }
                }
                "green" => {
                    if pair[0].parse::<i32>().unwrap() > 13 {
                        return false;
                    }
                }
                _ => {
                    panic!("should be blue red or green!");
                }
            }
        }
    }
    true
}

fn partb(line: &str) -> i32 {
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;
    let sets: Vec<&str> = line.split(&[':', ';'][..]).collect();
    //let sets: &str = line.split(":").collect::<Vec<&str>>()[1];
    for set in &sets[1..] {
        let parts: Vec<&str> = set.split(',').collect();
        for part in parts {
            let pair: Vec<&str> = part.split_whitespace().collect();
            match pair[1] {
                "blue" => {
                    b = max(pair[0].parse::<i32>().unwrap(), b);
                }
                "red" => {
                    r = max(pair[0].parse::<i32>().unwrap(), r);
                }
                "green" => {
                    g = max(pair[0].parse::<i32>().unwrap(), g);
                }
                _ => {
                    panic!("should be blue red or green!");
                }
            }
        }
    }
    r * g * b
}
