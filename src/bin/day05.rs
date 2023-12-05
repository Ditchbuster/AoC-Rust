#[allow(unused_imports)]
use std::{
    cmp::min,
    collections::HashMap,
    env,
    fs::{self},
};
use std::{str::Lines, usize};

fn main() {
    let day = 5;
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

    // let mut total: usize = 0;
    let mut lines_iter = contents.lines();
    let seeds: Vec<usize> = lines_iter
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    lines_iter.next();
    let seed_to_soil = parse_map(&mut lines_iter);
    let soil_to_fertilizer = parse_map(&mut lines_iter);
    let fertilizer_to_water = parse_map(&mut lines_iter);
    let water_to_light = parse_map(&mut lines_iter);
    let light_to_temp = parse_map(&mut lines_iter);
    let temp_to_hum = parse_map(&mut lines_iter);
    let hum_to_location = parse_map(&mut lines_iter);
    //let parts: Vec<&str> = line.split(&['|', ':'][..]).collect();

    let mut seed_loc: Vec<usize> = Vec::new();
    for s in &seeds {
        seed_loc.push(
            hum_to_location.convert(
                &temp_to_hum.convert(
                    &light_to_temp.convert(
                        &water_to_light.convert(
                            &fertilizer_to_water
                                .convert(&soil_to_fertilizer.convert(&seed_to_soil.convert(&s))),
                        ),
                    ),
                ),
            ),
        );
        //dbg!(seed_to_soil.convert(&s));
    }

    println!("Part A:{}", seed_loc.iter().min().unwrap());
    let mut min_loc = usize::MAX;
    let mut locs: Vec<usize> = Vec::new();
    for seed in seeds.chunks(2) {
        for x in 0..seed[1] {
            let source = seed[0] + x;
            if !locs.contains(&source) {
                locs.push(source);
                min_loc = min_loc.min(hum_to_location.convert(&temp_to_hum.convert(
                    &light_to_temp.convert(&water_to_light.convert(&fertilizer_to_water.convert(
                        &soil_to_fertilizer.convert(&seed_to_soil.convert(&(seed[0] + x))),
                    ))),
                )));
            }
        }
    }
    println!("Part B:{}", min_loc);
}
fn parse_map(lines: &mut Lines) -> Mapping {
    println!("Mapping: {}", lines.next().unwrap());
    Mapping {
        maps: lines
            .take_while(|s| !s.is_empty())
            .map(|s| {
                s.split_whitespace()
                    .take(3)
                    //.inspect(|x| println!("{}", x))
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()
            })
            .map(|v| Map {
                source_start: v[1],
                dest_start: v[0],
                length: v[2],
            })
            .collect(),
    }
}
#[derive(Debug)]
struct Mapping {
    maps: Vec<Map>,
}
impl Mapping {
    fn convert(&self, source: &usize) -> usize {
        for m in &self.maps {
            if source >= &m.source_start && source <= &(m.source_start + m.length) {
                return m.dest_start + (source - m.source_start);
            }
        }
        *source
    }
}

#[derive(Debug)]
struct Map {
    source_start: usize,
    dest_start: usize,
    length: usize,
}
