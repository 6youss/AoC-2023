use std::collections::{HashMap, HashSet};

use puzzle_input::PUZZLE_INPUT;

mod puzzle_input;

fn main() {
    let lines: Vec<&str> = PUZZLE_INPUT.lines().collect();
    let num_rows = lines.len();
    let num_cols: usize = lines[0].chars().count();
    let mut galaxis_positions: HashSet<(usize, usize)> = HashSet::new();

    let mut galaxis_rows: HashSet<usize> = HashSet::new();
    let mut galaxis_cols: HashSet<usize> = HashSet::new();

    for row in 0..num_rows {
        for col in 0..num_cols {
            if lines[row].chars().nth(col).unwrap() == '#' {
                galaxis_rows.insert(row);
                galaxis_cols.insert(col);
                galaxis_positions.insert((row, col));
            }
        }
    }

    let possible_pairs = find_pairs(&galaxis_positions.into_iter().collect());

    println!("{:?} {:?}", galaxis_rows, galaxis_cols);
}

fn find_pairs<T: Clone>(arr: &Vec<T>) -> Vec<(T, T)> {
    let mut pairs = Vec::new();

    for i in 0..arr.len() {
        for j in i + 1..arr.len() {
            pairs.push((arr[i].clone(), arr[j].clone()));
        }
    }

    pairs
}
