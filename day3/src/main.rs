use std::collections::HashMap;

use crate::engine_schematic::ENGINE_SCHEMATIC;

mod engine_schematic;

fn main() {
    let rows: Vec<&str> = ENGINE_SCHEMATIC.split('\n').collect();
    let num_rows = rows.len();
    let num_cols: usize = rows[0].chars().count();

    let mut potentiel_gears_map: HashMap<(usize, usize), u32> = HashMap::new();

    let mut sum_part_numbers = 0;
    let mut sum_gears_ratio = 0;

    let mut i = 0;
    while i < num_rows {
        let mut j = 0;
        while j < num_cols {
            let character = rows[i].chars().nth(j).unwrap();
            let is_digit = character.is_digit(10);
            if !is_digit {
                j += 1;
                continue;
            }
            let number_start_index = j;
            let mut number_end_index = j;

            loop {
                match rows[i].chars().nth(number_end_index + 1) {
                    Some(char) => {
                        if char.is_digit(10) {
                            number_end_index += 1;
                        } else {
                            break;
                        }
                    }
                    None => break,
                }
            }

            let number_value = rows[i]
                .get(number_start_index..(number_end_index + 1))
                .unwrap()
                .parse::<u32>()
                .unwrap();
            println!("number_value {}", number_value);

            // read points around the number
            let ii_start = if i == 0 { 0 } else { i - 1 };
            let ii_end = if i + 1 == num_rows { i } else { i + 1 };

            let jj_start = if number_start_index == 0 {
                0
            } else {
                number_start_index - 1
            };
            let jj_end = if number_end_index + 1 == num_cols {
                number_end_index
            } else {
                number_end_index + 1
            };
            println!("ii_start,jj_start {},{}", ii_start, jj_start);
            println!("ii_end,jj_end {},{}", ii_end, jj_end);
            println!(
                "start_number_index end_number_index {},{}",
                number_start_index, number_end_index
            );

            let mut is_part_number = false;
            'ii: for ii in ii_start..ii_end + 1 {
                for jj in jj_start..jj_end + 1 {
                    println!("==> {},{} ", ii, jj);
                    if let Some(character) = rows[ii].chars().nth(jj) {
                        if is_symbol(character) {
                            println!("found symbol {}", character);
                            is_part_number = true;

                            if character == '*' {
                                if let Some(value) = potentiel_gears_map.remove(&(ii, jj)) {
                                    sum_gears_ratio += value * number_value;
                                } else {
                                    potentiel_gears_map.insert((ii, jj), number_value);
                                }
                            }

                            break 'ii;
                        }
                    }
                }
            }

            if is_part_number {
                sum_part_numbers += number_value;
            }

            j += number_end_index - number_start_index + 1;
            println!("--------------------------------------------------");
        }
        i += 1;
    }
    println!("Sum of all part numbers: {}", sum_part_numbers);
    println!("Sum of all gears ratios: {}", sum_gears_ratio);
}

fn is_symbol(char: char) -> bool {
    return !char.is_digit(10) && !(char == '.');
}
