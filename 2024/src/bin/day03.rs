use regex::Regex;
use std::{
    env,
    fs::{self},
};

fn main() {
    let day = 3;
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

    let mut total = 0;
    for line in contents.lines() {
        //dbg!(line);
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        total += re
            .captures_iter(line)
            .map(|caps| {
                let (_, [x, y]) = caps.extract();
                let prod = x.parse::<i32>().unwrap() * y.parse::<i32>().unwrap();
                //dbg!(prod);
                prod
            })
            .sum::<i32>();
    }

    println!("Part A:{}", total);
    total = 0;
    let mut enb = true;
    for line in contents.lines() {
        
        let re = Regex::new(r"mul\((\d+),(\d+)\)|(don't\(\))|(do\(\))").unwrap();
        let caps = re.captures_iter(line);
        for c in caps{
            
            if c[0] == *"don't()"{
                enb = false;
            }else if c[0] == *"do()" {
                enb = true;
            }else if enb {
                //dbg!(&c);
                total += c[1].parse::<i32>().unwrap()*c[2].parse::<i32>().unwrap();
            }
            
        }
    }
    println!("Part B:{}", total);
}
