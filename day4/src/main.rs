use puzzle_input::PUZZLE_INPUT;
use std::time::{Duration, Instant};
mod puzzle_input;
fn main() {
    // Record the start time
    let start_time = Instant::now();

    // Perform the task or block of code for which you want to measure the execution time
    // For example, a time-consuming operation or function
    let mut sum = 0;
    for i in 0..100_000 {
        sum = 0;
        // Do some computation or task here
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
            if i % 1000 == 0 {
                print!("{}", i as u32);
            }
        }
    }
    println!("{}", sum);

    // Record the end time
    let end_time = Instant::now();

    // Calculate the duration by subtracting start time from end time
    let duration: Duration = end_time - start_time;

    // Convert the duration to milliseconds or other units if needed
    let duration_ms = duration.as_millis();

    println!("Execution took: {} milliseconds", duration_ms);
}
