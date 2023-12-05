use std::usize;
#[allow(unused_imports)]
use std::{
    cmp::max,
    collections::HashMap,
    env,
    fs::{self},
};

fn main() {
    let day = 5;
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

    let mut total: usize = 0;
    for line in contents.lines() {
        let parts: Vec<&str> = line.split(&['|', ':'][..]).collect();
    }
    println!("Part A:{}", total);
    //dbg!(&cards);
    total = 0;
    println!("Part B:{}", total);
}
