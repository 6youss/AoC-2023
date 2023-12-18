use std::collections::HashMap;

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
    let mut poly: Vec<(usize, usize)> = Vec::new();
    walk_pipe_lines(
        &lines,
        &graph,
        s_position,
        None,
        s_position,
        &mut visited,
        &mut steps,
        &mut poly,
    );

    println!("poly {:?}", poly);

    println!("steps {}", (steps + 1) / 2);
}

fn walk_pipe_lines(
    lines: &Vec<&str>,
    graph: &HashMap<(usize, usize), Vec<(usize, usize)>>,
    start: (usize, usize),
    parent: Option<(usize, usize)>,
    current_pos: (usize, usize),
    visited: &mut HashMap<(usize, usize), bool>,
    steps: &mut usize,
    poly: &mut Vec<(usize, usize)>,
) {
    // Mark the current position as visited
    *visited.entry(current_pos).or_insert(false) = true;
    let current_char = lines[current_pos.0].chars().nth(current_pos.1).unwrap();
    println!("Visited position: ({:?}) {}", current_pos, current_char);
    poly.push(current_pos);

    // Check neighbors of the current position
    if let Some(neighbors) = graph.get(&current_pos) {
        for &neighbor in neighbors {
            // Visit unvisited neighbors
            if !visited.get(&neighbor).cloned().unwrap_or(false) {
                *steps += 1;
                walk_pipe_lines(
                    lines,
                    graph,
                    start,
                    Some(current_pos),
                    neighbor,
                    visited,
                    steps,
                    poly,
                );
            } else if let Some(present_parent) = parent {
                if neighbor == start && present_parent != start {
                    println!("-----> Loop");
                }
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
