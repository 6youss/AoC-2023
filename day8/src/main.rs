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
        walk_maze(&maze, &instructions, &starting_nodes, 0)
    );
}

fn walk_maze(
    maze: &HashMap<&str, (&str, &str)>,
    instructions: &Vec<Direction>,
    current_nodes: &Vec<&str>,
    step: usize,
) -> usize {
    println!("Current node: {:?}", current_nodes);
    if all_end_with_z(current_nodes) {
        return step;
    }
    let next_direction_index = step % instructions.len();
    let next_direction = instructions[next_direction_index];

    let navigation_instructions: Vec<(&str, &str)> = current_nodes
        .iter()
        .map(|node| *maze.get(node).unwrap())
        .collect();
    let next_nodes = get_next_nodes(&navigation_instructions, next_direction);

    return walk_maze(maze, instructions, &next_nodes, step + 1);
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
    navigation_choices: &'a Vec<(&'a str, &'a str)>,
    current_direction: Direction,
) -> Vec<&'a str> {
    navigation_choices
        .iter()
        .map(|node| get_next_node(*node, current_direction))
        .collect()
}
