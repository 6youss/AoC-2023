use puzzle_input::ALMANAC;

use std::ops::Range;

mod puzzle_input;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Category {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}
#[derive(Debug, PartialEq, Eq, Hash)]
struct Node {
    category: Category,
    source: Range<u64>,
    dest: Range<u64>,
}

fn main() {
    let almanac_lines = ALMANAC.split("\n").filter(|&line| !line.trim().is_empty());

    let mut seeds: Vec<u64> = Vec::new();
    let mut currently_mapping = "";
    let mut graph: Vec<Node> = Vec::new();
    for line in almanac_lines {
        if line.contains("seeds:") {
            seeds = line
                .replace("seeds:", "")
                .split_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect();
        }
        let is_map_title_line = !line.chars().nth(0).unwrap().is_digit(10);
        if is_map_title_line {
            currently_mapping = line;
        } else {
            let map_values: Vec<u64> = line.split(" ").map(|n| n.parse::<u64>().unwrap()).collect();
            let dest = map_values[0];
            let source = map_values[1];
            let range_length = map_values[2];
            println!("{} {}", source, range_length);
            let source_range = source..source + range_length;
            let dest_range = dest..dest + range_length;
            if currently_mapping.contains("seed-to-soil") {
                let node: Node = Node {
                    category: Category::Seed,
                    source: source_range,
                    dest: dest_range,
                };
                graph.push(node);
            } else if currently_mapping.contains("soil-to-fertilizer") {
                let node: Node = Node {
                    category: Category::Soil,
                    source: source_range,
                    dest: dest_range,
                };
                graph.push(node);
            } else if currently_mapping.contains("fertilizer-to-water") {
                let node: Node = Node {
                    category: Category::Fertilizer,
                    source: source_range,
                    dest: dest_range,
                };
                graph.push(node);
            } else if currently_mapping.contains("water-to-light") {
                let node: Node = Node {
                    category: Category::Water,
                    source: source_range,
                    dest: dest_range,
                };
                graph.push(node);
            } else if currently_mapping.contains("light-to-temperature") {
                let node: Node = Node {
                    category: Category::Light,
                    source: source_range,
                    dest: dest_range,
                };
                graph.push(node);
            } else if currently_mapping.contains("temperature-to-humidity") {
                let node: Node = Node {
                    category: Category::Temperature,
                    source: source_range,
                    dest: dest_range,
                };
                graph.push(node);
            } else if currently_mapping.contains("humidity-to-location") {
                let node: Node = Node {
                    category: Category::Humidity,
                    source: source_range,
                    dest: dest_range,
                };
                graph.push(node);
            }
        }
    }

    let lowest = seeds
        .iter()
        .map(|seed| get_closest_location(&graph, &Category::Seed, *seed))
        .min()
        .unwrap();

    println!("Lowest {}", lowest);
}

fn get_closest_location(graph: &Vec<Node>, category: &Category, source_value: u64) -> u64 {
    let source_node = graph
        .iter()
        .find(|n| n.category == *category && n.source.contains(&source_value));

    let mut dest_value = source_value;
    if let Some(node) = source_node {
        dest_value = source_to_dest(&source_value, node);
    }

    let dest_category = match category {
        Category::Seed => Category::Soil,
        Category::Soil => Category::Fertilizer,
        Category::Fertilizer => Category::Water,
        Category::Water => Category::Light,
        Category::Light => Category::Temperature,
        Category::Temperature => Category::Humidity,
        Category::Humidity => Category::Location,
        Category::Location => Category::Location,
    };
    println!(
        "==> {:?} {} -> {:?} {}",
        category, source_value, dest_category, dest_value
    );

    if *category == Category::Location {
        return dest_value;
    }
    return get_closest_location(graph, &dest_category, dest_value);
}

fn source_to_dest(source_value: &u64, node: &Node) -> u64 {
    let dest = if node.source.contains(&source_value) {
        let diff = source_value - node.source.start;
        node.dest.start + diff
    } else {
        source_value.clone()
    };

    return dest;
}
