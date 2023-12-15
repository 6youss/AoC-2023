use std::collections::HashMap;

use crate::puzzle_input::PUZZLE_INPUT;

mod puzzle_input;

#[derive(Copy, Clone)]
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

    let mut maze: HashMap<&str, (&str, &str)> = HashMap::new();

    let mut starting_nodes: Vec<&str> = Vec::new();

    for line in lines.skip(1) {
        let parts: Vec<&str> = line.split(" = ").collect();

        let node = parts[0];
        let neighbours: Vec<&str> = parts[1]
            .trim_matches(|p| p == '(' || p == ')')
            .split(", ")
            .collect();
        maze.insert(node, (neighbours[0], neighbours[1]));
        if node.ends_with("A") {
            starting_nodes.push(node);
        }
    }
    println!("Starting nodes {:?}", starting_nodes);
    println!(
        "Walked {} steps",
        walk_maze(&maze, &instructions, &starting_nodes)
    );
}

fn walk_maze(
    maze: &HashMap<&str, (&str, &str)>,
    instructions: &Vec<Direction>,
    starting_nodes: &Vec<&str>,
) -> usize {
    let mut step: usize = 0;
    let mut current_nodes: Vec<&str> = starting_nodes.clone();
    let mut cycles: HashMap<&str, usize> = HashMap::new();
    loop {
        for (i, node) in current_nodes.iter().enumerate() {
            if node.ends_with('Z') {
                let corespendant_starting_node = starting_nodes[i];
                if !cycles.contains_key(corespendant_starting_node) {
                    cycles.insert(corespendant_starting_node, step);
                }
                println!("cycles {:?}", cycles);
            }
        }

        if cycles.keys().len() == current_nodes.len() {
            return find_lcm(cycles.values().clone().map(|e| *e as u64).collect()) as usize;
        }

        if all_end_with_z(&current_nodes) {
            return step;
        }
        let next_direction_index = step % instructions.len();
        let next_direction = instructions[next_direction_index];
        let navigation_choices = current_nodes
            .iter()
            .map(|node| *maze.get(node).unwrap())
            .collect();
        current_nodes = get_next_nodes(navigation_choices, next_direction);
        step += 1;
    }
}

fn all_end_with_z(navigation_choices: &Vec<&str>) -> bool {
    navigation_choices.iter().all(|node| node.ends_with("Z"))
}

fn get_next_node<'a>(navigation_choice: (&'a str, &'a str), current_direction: Direction) -> &str {
    match current_direction {
        Direction::L => navigation_choice.0,
        Direction::R => navigation_choice.1,
    }
}

fn get_next_nodes<'a>(
    navigation_choices: Vec<(&'a str, &'a str)>,
    current_direction: Direction,
) -> Vec<&'a str> {
    navigation_choices
        .iter()
        .map(|node| get_next_node(*node, current_direction))
        .collect()
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

fn find_lcm(numbers: Vec<u64>) -> u64 {
    let mut result = numbers[0];
    for &num in numbers.iter().skip(1) {
        result = lcm(result, num);
    }
    result
}
