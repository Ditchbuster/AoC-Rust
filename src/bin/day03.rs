#[allow(unused_imports)]
use std::{collections::HashMap, env, fs};

fn main() {
    let day = 2;
    let exm = false;

    println!("Hello, world! Day {}", day);
    let _args: Vec<String> = env::args().collect();

    let contents = fs::read_to_string(format!(
        "data\\{}\\day{}.txt",
        if exm { "example" } else { "input" },
        day
    ))
    .unwrap();

    for (i, line) in contents.lines().enumerate() {}
}
