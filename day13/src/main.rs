use std::collections::HashSet;

use puzzle_input::PUZZLE_INPUT;

mod puzzle_input;
fn main() {
    let mut horizontal_reflections: HashSet<(usize, usize, usize)> = HashSet::new();
    // let mut vertical_reflections: HashSet<(usize, usize, usize)> = HashSet::new();

    for (matrix, section) in PUZZLE_INPUT.split("\n\n").enumerate() {
        let lines = section.lines();
        let row_count = PUZZLE_INPUT.lines().count();
        let col_count = PUZZLE_INPUT.lines().nth(0).unwrap().len();

        let lines_vec = lines.collect::<Vec<&str>>();

        let reflections: Vec<(usize, usize)> = lines_vec
            .chunks(2)
            .enumerate()
            .filter(|ch| ch.1.len() == 2 && ch.1[0] == ch.1[1])
            .map(|ch| (ch.0 * 2, ch.0 * 2 + 1))
            .collect();

        let found_reflection = reflections.get(0).cloned();

        if let Some((i, ii)) = found_reflection {
            let mut row = 0;
            while row < row_count {
                let reflection_row = ii + (i - row);
                if reflection_row < row_count && lines.nth(row) != lines.nth(reflection_row) {
                    break;
                }
            }
            // all valid, then accept this as a good reflection
        }
    }
}
