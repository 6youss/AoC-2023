use std::collections::HashSet;

use puzzle_input::PUZZLE_INPUT;

mod puzzle_input;
fn main() {
    for (matrix, section) in PUZZLE_INPUT.split("\n\n").enumerate() {
        let row_count = PUZZLE_INPUT.lines().count();
        // let col_count = PUZZLE_INPUT.lines().nth(0).unwrap().len();

        let lines_vec = section.lines().collect::<Vec<&str>>();

        let reflections: Vec<(usize, usize)> = lines_vec
            .chunks(2)
            .enumerate()
            .filter(|(_, chunk)| chunk.len() == 2 && chunk[0] == chunk[1])
            .map(|ch| (ch.0 * 2, ch.0 * 2 + 1))
            .collect();

        let found_reflection = reflections.get(0).cloned();

        if let Some((i, ii)) = found_reflection {
            dbg!(matrix, i, ii);
            let mut row = 0;
            let mut is_valid = true;
            while row < row_count {
                if row == i {
                    break;
                }
                let reflection_row = ii + (i - row);
                let is_accepted_reflection = reflection_row >= row_count
                    || (reflection_row < row_count
                        && section.lines().nth(row).unwrap()
                            == section.lines().nth(reflection_row).unwrap());
                if !is_accepted_reflection {
                    is_valid = false;
                    break;
                }
                row += 1;
            }
            // all valid reflections, then accept this as a good reflection
            if is_valid {
                dbg!("aloors", i, ii);
            }
        }
    }
}
