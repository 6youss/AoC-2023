use std::collections::HashMap;

use crate::puzzle_input::PUZZLE_INPUT;

mod puzzle_input;

enum Direction {
    R,
    L,
}

fn main() {
    let lines = PUZZLE_INPUT.lines().filter(|line| !line.is_empty());
    // first line
    let instructions = lines
        .clone()
        .nth(0)
        .unwrap()
        .chars()
        .map(|c| match c {
            'R' => Direction::R,
            'L' => Direction::L,
            _ => panic!("Invalid direction"),
        })
        .collect::<Vec<Direction>>();

    let mut graph: HashMap<&str, (&str, &str)> = HashMap::new();

    for line in lines.skip(1) {
        let parts: Vec<&str> = line.split(" = ").collect();

        let node = parts[0];
        let neighbours: Vec<&str> = parts[1]
            .trim_matches(|p| p == '(' || p == ')')
            .split(", ")
            .collect();
        graph.insert(node, (neighbours[0], neighbours[1]));
    }
}
