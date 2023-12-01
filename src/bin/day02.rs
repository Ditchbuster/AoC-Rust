#[allow(unused_imports)]
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
}
