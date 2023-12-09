use puzzle_input::ALMANAC;
use std::collections::HashMap;
use std::ops::Range;

mod puzzle_input;
fn main() {
    let almanac_lines = ALMANAC.split("\n").filter(|&line| !line.trim().is_empty());

    let mut seeds: Vec<u32> = Vec::new();
    let mut currently_mapping = "";

    let mut seed_to_soil: HashMap<Range<u32>, Range<u32>> = HashMap::new();
    for line in almanac_lines {
        if line.contains("seeds:") {
            seeds = line
                .replace("seeds:", "")
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect();
        }
        let is_map_title_line = !line.chars().nth(0).unwrap().is_digit(10);
        if is_map_title_line {
            currently_mapping = line;
        } else {
            let map_values: Vec<u32> = line.split(" ").map(|n| n.parse::<u32>().unwrap()).collect();
            let source = map_values[0];
            let dest = map_values[1];
            let range_length = map_values[2];

            if currently_mapping.contains("seed-to-soil") {
                seed_to_soil.insert(source..source + range_length, dest..dest + range_length);
            }
        }
    }
    println!("{:?}", seeds);
}
