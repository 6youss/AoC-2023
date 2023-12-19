use std::collections::{HashMap, HashSet};

use puzzle_input::PUZZLE_INPUT;

mod puzzle_input;
fn main() {
    let mut graph: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    let lines: Vec<&str> = PUZZLE_INPUT.lines().collect();
    let num_rows = lines.len();
    let num_cols: usize = lines[0].chars().count();
    let mut s_position: (usize, usize) = (0, 0);
    let mut ss = vec!['|', '-', '7', 'F', 'J', 'L'];
    for row in 0..num_rows {
        for col in 0..num_cols {
            if lines[row].chars().nth(col).unwrap() == 'S' {
                s_position = (row, col);
            }
            let neighbours = get_pipe_neighbours(&lines, (row, col), &mut ss);
            graph.insert((row, col), neighbours);
        }
    }

    let real_s = ss[0];
    let real_puzzle_input = PUZZLE_INPUT.replace("S", &real_s.to_string());

    let real_lines: Vec<&str> = real_puzzle_input.lines().collect();
    let mut visited: HashMap<(usize, usize), bool> = HashMap::new();
    let mut steps = 0;
    let mut poly: HashSet<(usize, usize)> = HashSet::new();

    walk_pipe_lines(
        &real_lines,
        &graph,
        s_position,
        s_position,
        &mut visited,
        &mut steps,
        &mut poly,
        &mut ss,
    );

    println!("steps {} ⭐️", (steps + 1) / 2);

    let enclosed_tiles = get_tiles_enclosed_by_poly(&real_lines, &poly);

    println!("enclosed_tiles {:?} ⭐️⭐️", enclosed_tiles.len());
}

fn get_tiles_enclosed_by_poly(
    lines: &Vec<&str>,
    poly: &HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    let mut enclosed_tiles = HashSet::new();
    let num_rows = lines.len();
    let num_cols: usize = lines[0].chars().count();

    let mut min_row = usize::MAX;
    let mut min_col = usize::MAX;

    // Iterate through the vector to find the minimum col and row values
    for &(col, row) in poly {
        if col < min_col {
            min_col = col;
        }
        if row < min_row {
            min_row = row;
        }
    }

    for row in min_row..num_rows {
        for col in min_col..num_cols {
            let tile_pos = (row, col);
            if poly.contains(&tile_pos) {
                continue;
            }
            let mut intersect_right_counter = 0;
            let mut y = tile_pos.1;

            let mut previous_pointing: char = '?';
            // ray casting to right
            while y < num_cols {
                let raycast_pos = (tile_pos.0, y);
                if poly.contains(&raycast_pos) {
                    let current = lines[raycast_pos.0].chars().nth(raycast_pos.1).unwrap();
                    if current == '-' {
                        y += 1;
                        continue;
                    } else if current == '7' && previous_pointing == 'F' {
                        y += 1;
                        continue;
                    } else if current == 'F' {
                        previous_pointing = 'F';
                        y += 1;
                        continue;
                    } else if current == 'L' {
                        previous_pointing = 'L';
                        y += 1;
                        continue;
                    } else if current == 'J' && previous_pointing == 'L' {
                        y += 1;
                        continue;
                    }
                    intersect_right_counter += 1;
                    previous_pointing = '?';
                }
                y += 1;
            }

            if intersect_right_counter % 2 != 0 {
                enclosed_tiles.insert(tile_pos);
            }
        }
    }
    enclosed_tiles
}

fn walk_pipe_lines(
    lines: &Vec<&str>,
    graph: &HashMap<(usize, usize), Vec<(usize, usize)>>,
    start: (usize, usize),
    current_pos: (usize, usize),
    visited: &mut HashMap<(usize, usize), bool>,
    steps: &mut usize,
    poly: &mut HashSet<(usize, usize)>,
    ss: &mut Vec<char>,
) {
    // Mark the current position as visited
    *visited.entry(current_pos).or_insert(false) = true;

    poly.insert(current_pos);

    if let Some(neighbors) = graph.get(&current_pos) {
        for &neighbor in neighbors {
            if !visited.get(&neighbor).cloned().unwrap_or(false) {
                *steps += 1;
                walk_pipe_lines(lines, graph, start, neighbor, visited, steps, poly, ss);
            }
        }
    }
}

fn get_pipe_neighbours(
    lines: &Vec<&str>,
    pipe_pos: (usize, usize),
    ss: &mut Vec<char>,
) -> Vec<(usize, usize)> {
    let rows = lines.len();
    let cols: usize = lines[0].chars().count();
    let source_char = lines[pipe_pos.0].chars().nth(pipe_pos.1).unwrap();
    let mut neighbors = Vec::new();

    // North neighbor
    if pipe_pos.0 > 0 && source_can_go_north(source_char) {
        let north = (pipe_pos.0 - 1, pipe_pos.1);
        let north_char = lines[north.0].chars().nth(north.1).unwrap();
        if north_char != '.' {
            neighbors.push(north);
        }

        // if the north character can be accessible from S
        if source_char == 'S' && vec!['|', '7', 'F'].contains(&north_char) {
            // set S to be one of the characters that can go north
            *ss = array_intersection(ss, &vec!['L', '|', 'J']);
        }
    }
    // East neighbor
    if pipe_pos.1 < cols - 1 && source_can_go_east(source_char) {
        let east = (pipe_pos.0, pipe_pos.1 + 1);
        let east_char = lines[east.0].chars().nth(east.1).unwrap();
        if east_char != '.' {
            neighbors.push(east);
        }
        // if the east character can be accessible from S
        if source_char == 'S' && vec!['-', '7', 'J'].contains(&east_char) {
            // set S to be one of the characters that can go east
            *ss = array_intersection(ss, &vec!['-', 'F', 'L']);
        }
    }
    // South neighbor
    if pipe_pos.0 < rows - 1 && source_can_go_south(source_char) {
        let south = (pipe_pos.0 + 1, pipe_pos.1);
        let southchar = lines[south.0].chars().nth(south.1).unwrap();
        if southchar != '.' {
            neighbors.push(south);
        }
        // if the South character can be accessible from S
        if source_char == 'S' && vec!['|', 'L', 'J'].contains(&southchar) {
            // set S to be one of the characters that can go South
            *ss = array_intersection(ss, &vec!['F', '7', '|']);
        }
    }
    // West neighbor
    if pipe_pos.1 > 0 && source_can_go_west(source_char) {
        let west = (pipe_pos.0, pipe_pos.1 - 1);
        let west_char = lines[west.0].chars().nth(west.1).unwrap();
        if west_char != '.' {
            neighbors.push(west);
        }
        // if the West character can be accessible from S
        if source_char == 'S' && vec!['F', 'L', '-'].contains(&west_char) {
            // set S to be one of the characters that can go West
            *ss = array_intersection(ss, &vec!['-', '7', 'J']);
        }
    }

    neighbors
}

fn source_can_go_north(source_char: char) -> bool {
    vec!['S', '|', 'J', 'L'].contains(&source_char)
}
fn source_can_go_east(source_char: char) -> bool {
    vec!['S', '-', 'L', 'F'].contains(&source_char)
}
fn source_can_go_south(source_char: char) -> bool {
    vec!['S', 'F', '|', '7'].contains(&source_char)
}
fn source_can_go_west(source_char: char) -> bool {
    vec!['S', '-', 'J', '7'].contains(&source_char)
}

fn array_intersection<T: PartialEq + Clone>(array1: &Vec<T>, array2: &Vec<T>) -> Vec<T> {
    let intersection: Vec<_> = array1
        .iter()
        .filter(|&x| array2.contains(x))
        .cloned()
        .collect();

    intersection
}
