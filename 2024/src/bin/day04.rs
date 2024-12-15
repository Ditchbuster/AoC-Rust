
use std::{
    char, env,
    fs::{self},
};

fn main() {
    let day = 4;
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
    let mut grid = Vec::new();
    for line in contents.lines() {
        //dbg!(line);
        grid.push(line.chars().collect::<Vec<char>>());
    }

    for y in 0..grid.len() {
        //print!("{}",check_grid(&grid, grid[0][i], 0, i));
        for x in 0..grid[y].len(){
            total += total_xmas(&grid, y, x)
        }
        //dbg!((y,total));
    }

    println!("Part A:{}", total);
    total = 0;
    for y in 0..grid.len() {
        //print!("{}",check_grid(&grid, grid[0][i], 0, i));
        for x in 0..grid[y].len(){
            if check_mas(&grid, y, x){
                total += 1;
            }
        }
        //dbg!((y,total));
    }
    println!("Part B:{}", total);
}
fn check_grid(grid: &Vec<Vec<char>>, letter: char, y: usize, x: usize) -> bool {
    if y >= grid.len() || x >= grid[0].len() {
        return false;
    } else {
        return grid[y][x] == letter;
    }
}
fn total_xmas(grid: &Vec<Vec<char>>, y: usize, x: usize) -> usize {
    let mut total = 0;
    if grid[y][x] == 'X' {
        for d in 0..=7 {
            if check_direction(grid, y, x, d){
                total += 1;
            }
        }
    }
    return total;
}
fn check_direction(grid: &Vec<Vec<char>>, y: usize, x: usize, dir: usize) -> bool {

    // dir 0 is up
    // assumes y,x has aready been checked for 'x'
    let xmas = ['X', 'M', 'A', 'S'];
    let d: (isize, isize) = match dir {
        0 => {
            if y < 3 {
                return false;
            }
            (-1, 0)
        }
        1 => {
            if y < 3 || x >= grid[y].len() - 3 {
                return false;
            }
            (-1, 1)
        }
        2 => {
            if x >= grid[y].len() - 3 {
                return false;
            }
            (0, 1)
        }
        3 => {
            if y >= grid.len() - 3 || x >= grid[y].len() - 3 {
                return false;
            }
            (1, 1)
        }
        4 => {
            if y >= grid.len() - 3 {
                return false;
            }
            (1, 0)
        }
        5 => {
            if y >= grid.len() - 3 || x < 3 {
                return false;
            }
            (1, -1)
        }
        6 => {
            if x < 3 {
                return false;
            }
            (0, -1)
        }
        7 => {
            if y < 3 || x < 3 {
                return false;
            }
            (-1, -1)
        }
        _ => panic!(),
    };
    for i in 1..=3 as isize {
        let iy = y.checked_add_signed(i.checked_mul(d.0).unwrap()).unwrap();
        let ix = x.checked_add_signed(i.checked_mul(d.1).unwrap()).unwrap();
        if xmas[i as usize] != grid[iy][ix] {
            return false;
        };
    }
    //dbg!((y,x));
    return true;
}
fn check_mas(grid: &Vec<Vec<char>>, y: usize,x: usize) -> bool{
    if y < 1 || y > grid.len()-2 || x < 1 || x > grid[y].len()-2 || grid[y][x] != 'A'{ return false;}
    else {
        let tl = grid[y-1][x-1]; //top left ... just to make lower easier to read
        let tr = grid[y-1][x+1];
        let br = grid[y+1][x+1];
        let bl = grid[y+1][x-1];
        let bs = (tl == 'M' && br == 'S') || (tl == 'S' && br == 'M');//backslash is valid
        let fs = (tr == 'M' && bl == 'S') || (tr == 'S' && bl == 'M');//forwardslash
        return fs && bs;
    }
}
