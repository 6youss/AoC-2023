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

    for line in lines.skip(1) {
        let parts: Vec<&str> = line.split(" = ").collect();

        let node = parts[0];
        let neighbours: Vec<&str> = parts[1]
            .trim_matches(|p| p == '(' || p == ')')
            .split(", ")
            .collect();
        maze.insert(node, (neighbours[0], neighbours[1]));
    }
    println!("Walked {} steps", walk_maze(&maze, &instructions, "AAA", 0));
}

fn walk_maze(
    maze: &HashMap<&str, (&str, &str)>,
    instructions: &Vec<Direction>,
    current_node: &str,
    step: usize,
) -> usize {
    println!("Current node: {}", current_node);
    if current_node == "ZZZ" {
        return step;
    }
    let next_direction_index = step % instructions.len();
    let next_direction = instructions[next_direction_index];
    let navigation_instruction = maze.get(current_node).unwrap();
    let next_node = get_next_node(*navigation_instruction, next_direction);

    return walk_maze(maze, instructions, next_node, step + 1);
}

fn get_next_node<'a>(current_node: (&'a str, &'a str), current_direction: Direction) -> &str {
    match current_direction {
        Direction::L => current_node.0,
        Direction::R => current_node.1,
    }
}
