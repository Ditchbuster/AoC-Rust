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
    let day = 6;
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
    let mut lines = contents.lines();
    let time = lines.next().unwrap().split_whitespace().skip(1);
    let dis = lines.next().unwrap().split_whitespace().skip(1);
    let races: Vec<Race> = time
        .zip(dis)
        .map(|(t, d)| Race {
            time: t.parse::<usize>().unwrap(),
            dist: d.parse::<usize>().unwrap(),
        })
        .collect();
    //dbg!(&races);
    /* let perm = time.zip(dis).map(|(t, d)| Race {
        time: t.parse::<usize>().unwrap(),
        dist: d.parse::<usize>().unwrap(),
    }); */
    let temp = races
        .iter()
        .map(|r| r.calc_wins())
        //.inspect(|r| println!("{}", r.0 - r.1 - 1))
        .fold(1, |acc, r| acc * (r.0 - r.1 - 1));

    println!("Part A:{}", temp);
    let mut lines = contents.lines();
    let time = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.to_string())
        .reduce(|cur: String, nxt: String| cur + &nxt)
        .unwrap()
        .parse()
        .unwrap();
    let dist = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.to_string())
        .reduce(|cur: String, nxt: String| cur + &nxt)
        .unwrap()
        .parse()
        .unwrap();
    let races = Race { time, dist };
    let wins = races.calc_wins();
    println!("Part B:{}", wins.0 - wins.1 - 1);
}

#[derive(Debug)]
struct Race {
    time: usize,
    dist: usize,
}
impl Race {
    fn calc_wins(&self) -> (usize, usize) {
        let root1 = 0.5
            * (f64::sqrt(self.time.pow(2) as f64 - 4 as f64 * self.dist as f64) + self.time as f64);
        let root2 = 0.5
            * (self.time as f64 - f64::sqrt(self.time.pow(2) as f64 - 4 as f64 * self.dist as f64));
        dbg!(&root1, &root2);
        (root1.ceil() as usize, root2.floor() as usize)
    }
}
