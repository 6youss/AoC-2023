mod calibration_document;
use calibration_document::CALIBRATION_DOCUMENT;
use std::collections::HashMap;
fn main() {
    let mut number_map: HashMap<&str, u32> = HashMap::new();
    number_map.insert("one", 1);
    number_map.insert("two", 2);
    number_map.insert("three", 3);
    number_map.insert("four", 4);
    number_map.insert("five", 5);
    number_map.insert("six", 6);
    number_map.insert("seven", 7);
    number_map.insert("eight", 8);
    number_map.insert("nine", 9);

    let mut total_sum = 0;
    let mut line_calibration_value = 0;

    let mut latest_found_digit = 0;
    let mut potential_letters_digit = String::from("");
    for c in CALIBRATION_DOCUMENT.chars() {
        let mut is_digit: bool = c.is_digit(10);
        if is_digit {
            latest_found_digit = c.to_digit(10).unwrap();
        } else {
            potential_letters_digit.push(c);
            if let Some((_, value)) = number_map
                .iter()
                .find(|(key, _)| potential_letters_digit.contains(**key))
            {
                potential_letters_digit = String::from(c);
                latest_found_digit = *value;
                is_digit = true;
            }
        }
        if is_digit && line_calibration_value == 0 {
            line_calibration_value += latest_found_digit * 10;
        }
        let is_new_line: bool = c == '\n';
        if is_new_line {
            line_calibration_value += latest_found_digit;
            total_sum += line_calibration_value;

            // reset for next line
            line_calibration_value = 0;
            potential_letters_digit = String::from("");
        }
    }
    println!("The total sum is: {}", total_sum);
}
