use std::{
    collections::HashMap,
    env,
    fs::{self},
};

fn main() {
    let day = 1;
    let exm = false;

    println!("Hello, world! Day 1");
    let _args: Vec<String> = env::args().collect();
    //dbg!(args);

    let contents = fs::read_to_string(format!(
        "data\\{}\\day{}.txt",
        if exm { "example" } else { "input" },
        day
    ))
    .unwrap();
    let mut total = 0;
    for line in contents.lines() {
        let parts = line.split(" ").collect::<Vec<_>>();
        //println!("{:?}", parts);
        total += first_char_to_int(parts[0]);
    }
    println!("Part A:{}", total);
    total = 0;
    for line in contents.lines() {
        let parts = line.split(" ").collect::<Vec<_>>();
        total += firstlast_word_or_digit_to_int(parts[0]);
    }
    println!("Part B:{}", total);
}
fn firstlast_word_or_digit_to_int(inp: &str) -> i32 {
    let numbers = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let mut pos = inp.len();
    let mut key = "";
    for (k, _) in &numbers {
        let i = inp.find(k).unwrap_or(pos);
        if i < pos {
            pos = i;
            key = k;
        }
    }

    let i = inp.find(|c: char| c.is_numeric()).unwrap_or(pos);
    let firstval: i32;
    if i < pos {
        firstval = inp[i..i + 1].parse::<i32>().unwrap();
    } else {
        firstval = numbers[key];
    }
    pos = 0;
    for (k, _) in &numbers {
        let i = inp.rfind(k).unwrap_or(pos);
        if i > pos {
            pos = i;
            key = k;
        }
    }

    let i = inp.rfind(|c: char| c.is_numeric()).unwrap_or(pos);
    let lastval: i32;
    if i >= pos {
        lastval = inp[i..i + 1].parse::<i32>().unwrap();
    } else {
        if key == "" {
            dbg!(inp, key, pos, i);
        }
        lastval = numbers[key];
    }
    //dbg!(lastval + (firstval * 10));
    lastval + (firstval * 10)
}
fn first_char_to_int(inp: &str) -> i32 {
    let mut code = String::new();
    for a in inp.chars() {
        if a.is_numeric() {
            code.push_str(&a.to_string());
            break;
        }
    }
    for a in inp.chars().rev() {
        if a.is_numeric() {
            code.push_str(&a.to_string());
            break;
        }
    }
    //dbg!(code);
    code.parse::<i32>().unwrap()
}
