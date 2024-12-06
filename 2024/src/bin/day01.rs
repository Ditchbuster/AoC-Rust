use std::{
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
    let mut first_parts: Vec<i32> = Vec::new();
    let mut second_parts: Vec<i32> = Vec::new();
    for line in contents.lines() {
       if let Some((x,y)) = line.split_once(" "){
            //println!("{:?}", (x,y));
            first_parts.push(x.parse::<i32>().unwrap());
            second_parts.push(y.trim().parse::<i32>().unwrap());
        }
        //total += first_char_to_int(parts[0]);
    }
    first_parts.sort();
    second_parts.sort();
    //dbg!(first_parts.iter().zip(second_parts.iter()).map(|t| (t.0-t.1).abs()).sum::<i32>());
    let total = first_parts.iter().zip(second_parts.iter()).map(|t| (t.0-t.1).abs()).sum::<i32>();
    println!("Part A:{}", total);
    //dbg!(first_parts,second_parts);
    let total = first_parts.iter().map(|i| second_parts.iter().filter(|j| *j==i).count() as i32 * i).sum::<i32>();
    println!("Part B:{}", total);
}

