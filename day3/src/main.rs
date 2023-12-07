use crate::engine_schematic::ENGINE_SCHEMATIC;

mod engine_schematic;

fn main() {
    let lines: Vec<&str> = ENGINE_SCHEMATIC.trim().split('\n').collect();
    let num_rows = lines.len();
    let num_cols = lines[0].chars().count();

    for i in 0..num_rows {
        for j in 0..num_cols {
            let character = lines[i].chars().nth(j).unwrap();
            let is_digit = character.is_digit(10);
            let is_symbol = !is_digit && !(character == '.');
        }
    }
}
