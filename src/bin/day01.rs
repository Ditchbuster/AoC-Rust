use std::{
    env,
    fs::{self, File},
};

fn main() {
    let day = 1;
    let exm = true;

    println!("Hello, world! Day 1");
    let args: Vec<String> = env::args().collect();
    //dbg!(args);

    let contents = fs::read_to_string(format!(
        "data\\{}\\day{}.txt",
        if exm { "example" } else { "input" },
        day
    ))
    .unwrap();
    println!("{}", contents);

    for line in contents.lines() {
        let parts = line.split(" ");
    }
}
