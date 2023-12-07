mod engine_schematic;

fn main() {
    let lines: Vec<&str> = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#
        .split('\n')
        .collect();
    let num_rows = lines.len();
    let num_cols = lines[0].chars().count();
    let mut sum_part_numbers = 0;
    let mut i = 0;
    while i < num_rows {
        let mut j = 0;
        println!("",);
        while j < num_cols {
            let character = lines[i].chars().nth(j).unwrap();
            let is_digit = character.is_digit(10);
            if !is_digit {
                j += 1;
                continue;
            }
            let start_number_index = j;
            let mut end_number_index = j;
            let mut is_neighbour_digit = true;

            while is_neighbour_digit {
                match lines[i].chars().nth(end_number_index) {
                    Some(char) => {
                        if char.is_digit(10) {
                            end_number_index += 1;
                        } else {
                            is_neighbour_digit = false;
                        }
                    }
                    None => is_neighbour_digit = false,
                }
            }
            j += end_number_index;
            // read points around the number
            // let mut is_part_number = false;

            // // let ii_start = if i == 0 { 0 } else { i - 1 };
            // // let ii_end = if i + 1 > num_rows { num_rows } else { i + 1 };
            // // 'ii: for ii in ii_start..ii_end {
            // //     let jj_start = if start_number_index == 0 {
            // //         0
            // //     } else {
            // //         start_number_index - 1
            // //     };
            // //     let jj_end = if end_number_index + 1 > num_cols {
            // //         num_cols
            // //     } else {
            // //         end_number_index + 1
            // //     };

            // //     for jj in jj_start..jj_end {
            // //         if let Some(character) = lines[ii].chars().nth(jj) {
            // //             if is_symbol(character) {
            // //                 is_part_number = true;
            // //                 break 'ii;
            // //             }
            // //         }
            // //     }
            // // }
            let part_number_value = lines[i]
                .get(start_number_index..end_number_index)
                .unwrap()
                .parse::<u32>()
                .unwrap();
            println!("{}", part_number_value);
            // if is_part_number {
            //     sum_part_numbers += part_number_value;
            // }
            j += 1;
        }
        i += 1;
    }
    println!("Sum of all part numbers: {}", sum_part_numbers);
}

fn is_symbol(char: char) -> bool {
    return !char.is_digit(10) && !(char == '.');
}
