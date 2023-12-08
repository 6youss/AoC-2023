use puzzle_input::PUZZLE_INPUT;
use std::time::{Duration, Instant};
mod puzzle_input;
fn main() {
    let start_time = Instant::now();

    for i in 0..1 {
        if i % 1000 == 0 {
            println!("benchmark index {}", i);
        }

        let mut sum = 0;
        for card in PUZZLE_INPUT.split('\n') {
            let table_row: Vec<&str> = card.split(":").collect::<Vec<&str>>();

            let numbers_list = table_row[1].split("|").collect::<Vec<&str>>();
            let winning: Vec<i32> = numbers_list[0]
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            let owned: Vec<i32> = numbers_list[1]
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();

            let mut number_owned_winning = 0;

            for &element in &owned {
                if winning.contains(&element) {
                    number_owned_winning += 1;
                }
            }

            let score = u32::pow(2, number_owned_winning) / 2;

            sum += score;
        }
        println!("Final sum {}", sum);
    }

    let end_time = Instant::now();
    let duration: Duration = end_time - start_time;
    let duration_ms = duration.as_millis();
    println!("Execution took: {} milliseconds", duration_ms);
}
