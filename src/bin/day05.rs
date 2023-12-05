#[allow(unused_imports)]
use std::{
    cmp::min,
    collections::HashMap,
    env,
    fs::{self},
};
use std::{collections::hash_map::RandomState, ops::Range, str::Lines, usize};

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
    let mut ranges: &mut Vec<Range<usize>> = &mut Vec::new();
    for seed in seeds.chunks(2) {
        ranges.push(seed[0]..seed[0] + seed[1]);
    }
    let my_mappings = vec![
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temp,
        temp_to_hum,
        hum_to_location,
    ];

    for map in my_mappings {
        let dest: &mut Vec<Range<usize>> = &mut Vec::new();
        while let Some(range) = &ranges.pop() {
            let conv = map.convert_range(range.clone());
            dest.push(conv.0);
            if let Some(r) = conv.1 {
                ranges.push(r);
            }
        }
        *ranges = dest.clone();
    }
    //dbg!(ranges);
    println!(
        "Part B:{}",
        ranges.iter().map(|s| { s.start }).min().unwrap()
    );
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
            if source >= &m.source_start && source < &(m.source_start + m.length) {
                return m.dest_start + (source - m.source_start);
            }
        }
        *source
    }
    fn convert_range(&self, range: Range<usize>) -> (Range<usize>, Option<Range<usize>>) {
        for m in &self.maps {
            if range.start >= m.source_start && range.start < (m.source_start + m.length) {
                if range.end <= (m.source_start + m.length) {
                    //dbg!(&range, &m);
                    return (
                        m.dest_start + (range.start - m.source_start)
                            ..m.dest_start + (range.end - m.source_start),
                        None,
                    );
                }
                return (
                    m.dest_start + (range.start - m.source_start)..m.dest_start + m.length,
                    Some(m.source_start + m.length..range.end),
                );
            }
        }
        // the starting range was not inside any mapping
        let mut maps_in_range: Vec<&Map> = Vec::new();
        for m in &self.maps {
            if m.source_start > range.start && m.source_start < range.end {
                maps_in_range.push(m)
            }
        }
        if maps_in_range.len() != 0 {
            let maps_min = maps_in_range.iter().map(|m| m.source_start).min().unwrap();
            return (range.start..maps_min, Some(maps_min..range.end));
        }
        (range, None)
    }
}

#[derive(Debug)]
struct Map {
    source_start: usize,
    dest_start: usize,
    length: usize,
}
