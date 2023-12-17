use std::collections::HashMap;

use puzzle_input::PUZZLE_INPUT;

mod puzzle_input;
fn main() {
    let mut graph: HashMap<&char, Vec<&char>> = HashMap::new();
    let rows: Vec<&str> = PUZZLE_INPUT.lines().collect();
    let num_rows = rows.len();
    let num_cols: usize = rows[0].chars().count();

    // fill graph
    for i in 0..num_rows {
        for j in 0..num_cols {
            let character = rows[i].chars().nth(j).unwrap();
        }
    }
}
