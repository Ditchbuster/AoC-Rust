use std::{
    env,
    fs::{self}, iter::Enumerate,
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

    let mut total = 0;
    for line in contents.lines() {
        let parts = line
            .split(" ")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i32>>();
        if is_safe(&parts){
            total += 1;
        }
        //total += first_char_to_int(parts[0]);
    }

    println!("Part A:{}", total);
    total = 0;
    for line in contents.lines() {
        let parts = line
            .split(" ")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i32>>();
        if is_safe2(parts){
            total += 1;
        }
        //total += first_char_to_int(parts[0]);
    }
    println!("Part B:{}", total);
}

fn is_safe(codes: &Vec<i32>) -> bool{
    if (codes.is_sorted() || codes.iter().rev().is_sorted()) {
        for w in codes.windows(2){
            let dif = w[0]-w[1];
            if 3<dif || dif < -3 || dif == 0{
                return false;
            }
        }
        return true;
    }
    return false;
}

fn is_safe2(codes: Vec<i32>) -> bool{
    if is_safe(&codes){
        return true;
    }
    for i in 0..codes.len(){
        let mut temp  = codes.clone();
        temp.remove(i);
        if is_safe(&temp){
            return true;
        }

    }
    return false;
}

