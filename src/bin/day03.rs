#[allow(unused_imports)]
use std::{cmp::max, collections::HashMap, env, fs};
use std::{collections::HashSet, ops::Sub};

fn main() {
    let day = 3;
    let exm = false;

    println!("Hello, world! Day {}", day);
    let _args: Vec<String> = env::args().collect();

    let contents = fs::read_to_string(format!(
        "data\\{}\\day{}.txt",
        if exm { "example" } else { "input" },
        day
    ))
    .unwrap();

    let mut parts: Vec<Part> = Vec::new();
    let mut symb: Vec<(usize, usize)> = Vec::new();
    for (i, line) in contents.lines().enumerate() {
        /* for (x, c) in line.chars().enumerate() {
            if c.is_numeric() {
                let f = line[x..]
                    .find(|c: char| !c.is_numeric())
                    .unwrap_or(line.len());
                dbg!((i, x, c, f));
                parts.push(Part {
                    x: x,
                    y: i,
                    number: line[x..f].parse::<i32>().unwrap(),
                });
            }
        } */
        let numbers: HashSet<&str> = line
            .split(|x: char| x.is_ascii_punctuation())
            .filter(|&x| !x.is_empty())
            .collect();
        for s in &numbers {
            let m: Vec<(usize, &str)> = line.match_indices(s).collect();
            for (j, p) in m {
                if (j == 0 || line[j - 1..].starts_with(|c: char| !c.is_digit(10)))
                    && (j + s.len() == line.len()
                        || line[j + s.len()..].starts_with(|c: char| !c.is_digit(10)))
                {
                    parts.push(Part {
                        x: j,
                        y: i,
                        number: p.parse::<i32>().unwrap(),
                        length: s.len(),
                    });
                }
            }
        }

        for (x, c) in line.chars().enumerate() {
            if !c.is_digit(10) && c != '.' {
                symb.push((x, i));
            }
        }
    }
    let mut total: i32 = 0;
    for part in parts {
        if check_part(&part, &symb) {
            print!("{}:", part.number);
            if part.number == 354 {
                print!("({},{})", part.x, part.y);
            }
            total += part.number;
        }
    }
    //dbg!(symb);
    println!("Part A:{}", total);
}
fn check_part(part: &Part, symb: &Vec<(usize, usize)>) -> bool {
    let minx = if part.x == 0 { 0 } else { part.x - 1 }; //handle neg for usize
    let miny = if part.y == 0 { 0 } else { part.y - 1 };
    for i in minx..(part.x + part.length + 1) {
        for j in miny..(part.y + 2) {
            if symb.contains(&(i, j)) {
                if part.number == 354 {
                    dbg!((minx, part.x + part.length + 1, miny, part.y + 1));
                }
                return true;
            }
        }
    }
    if part.number == 354 {
        dbg!("false", (minx, part.x + part.length + 1, miny, part.y + 1));
    }
    false
}
#[derive(Debug)]
struct Part {
    x: usize,
    y: usize, //starting point
    number: i32,
    length: usize,
}
