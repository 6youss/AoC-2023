use std::collections::{HashMap, HashSet};

use puzzle_input::PUZZLE_INPUT;

mod puzzle_input;
fn main() {
    let mut graph: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    let lines: Vec<&str> = PUZZLE_INPUT.lines().collect();
    let num_rows = lines.len();
    let num_cols: usize = lines[0].chars().count();
    let mut s_position: (usize, usize) = (0, 0);
    for row in 0..num_rows {
        for col in 0..num_cols {
            if lines[row].chars().nth(col).unwrap() == 'S' {
                s_position = (row, col);
            }
            let neighbours = get_pipe_neighbours(&lines, (row, col));
            graph.insert((row, col), neighbours);
        }
    }

    let mut visited: HashMap<(usize, usize), bool> = HashMap::new();
    let mut steps = 0;
    let mut poly: HashSet<(usize, usize)> = HashSet::new();

    walk_pipe_lines(
        &lines,
        &graph,
        s_position,
        s_position,
        &mut visited,
        &mut steps,
        &mut poly,
    );

    // println!("poly {:?}", poly);

    // println!("steps {}", (steps + 1) / 2);

    let enclosed_tiles = get_tiles_enclosed_by_poly(&lines, &poly);

    // println!("enclosed_tiles {:?}", enclosed_tiles);
}

fn get_tiles_enclosed_by_poly(
    lines: &Vec<&str>,
    poly: &HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    let mut enclosed_tiles = HashSet::new();
    let num_rows = lines.len();
    let num_cols: usize = lines[0].chars().count();
    // Initialize min_col and min_row with the maximum possible value for usize
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
            for y in tile_pos.1..num_cols {
                if poly.contains(&(tile_pos.0, y)) {
                    intersect_right_counter += 1;
                }
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
) {
    // Mark the current position as visited
    *visited.entry(current_pos).or_insert(false) = true;

    poly.insert(current_pos);

    if let Some(neighbors) = graph.get(&current_pos) {
        for &neighbor in neighbors {
            if !visited.get(&neighbor).cloned().unwrap_or(false) {
                *steps += 1;
                walk_pipe_lines(lines, graph, start, neighbor, visited, steps, poly);
            }
        }
    }
}

fn get_pipe_neighbours(lines: &Vec<&str>, pipe_pos: (usize, usize)) -> Vec<(usize, usize)> {
    let rows = lines.len();
    let cols: usize = lines[0].chars().count();
    let source_char = lines[pipe_pos.0].chars().nth(pipe_pos.1).unwrap();
    let mut neighbors = Vec::new();

    // North neighbor
    if pipe_pos.0 > 0 && vec!['S', '|', 'J', 'L'].contains(&source_char) {
        let north = (pipe_pos.0 - 1, pipe_pos.1);
        let north_char = lines[north.0].chars().nth(north.1).unwrap();
        if north_char != '.' {
            neighbors.push(north);
        }
    }
    // West neighbor
    if pipe_pos.1 > 0 && vec!['S', '-', 'J', '7'].contains(&source_char) {
        let west = (pipe_pos.0, pipe_pos.1 - 1);
        let west_char = lines[west.0].chars().nth(west.1).unwrap();
        if west_char != '.' {
            neighbors.push(west);
        }
    }
    // South neighbor
    if pipe_pos.0 < rows - 1 && vec!['S', 'F', '|', '7'].contains(&source_char) {
        let south = (pipe_pos.0 + 1, pipe_pos.1);
        let southchar = lines[south.0].chars().nth(south.1).unwrap();
        if southchar != '.' {
            neighbors.push(south);
        }
    }
    // East neighbor
    if pipe_pos.1 < cols - 1 && vec!['S', '-', 'L', 'F'].contains(&source_char) {
        let east = (pipe_pos.0, pipe_pos.1 + 1);
        let east_char = lines[east.0].chars().nth(east.1).unwrap();
        if east_char != '.' {
            neighbors.push(east);
        }
    }
    neighbors
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_neighbors() {
        let lines = "abc
def
ijK";
        assert_eq!(
            get_pipe_neighbours(&lines.lines().collect(), (0, 0)),
            vec![(0, 0), (0, 0)]
        );
        assert_eq!(
            get_pipe_neighbours(&lines.lines().collect(), (1, 1)),
            vec![(0, 0), (0, 0), (0, 0), (0, 0)]
        );
    }
}
