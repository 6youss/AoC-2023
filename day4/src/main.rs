use puzzle_input::PUZZLE_INPUT;
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};
mod puzzle_input;
fn main() {
    let start_time = Instant::now();

    for i in 0..1 {
        if i % 1000 == 0 {
            println!("benchmark execution arrived at {}", i);
        }

        let mut sum = 0;

        let mut cards_count_map: HashMap<usize, u32> = HashMap::new();

        for (card_index, card) in PUZZLE_INPUT.split('\n').enumerate() {
            let table_row: Vec<&str> = card.split(":").collect::<Vec<&str>>();
            let numbers_list = table_row[1].split("|").collect::<Vec<&str>>();
            let winning_numbers: Vec<i32> = numbers_list[0]
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            let owned_numbers: Vec<i32> = numbers_list[1]
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();

            let mut matching_numbers_count = 0;

            for &owned_number in &owned_numbers {
                if winning_numbers.contains(&owned_number) {
                    matching_numbers_count += 1;
                }
            }

            let card_count = cards_count_map.entry(card_index).or_insert(1).clone();

            for i in (card_index + 1)..(card_index + matching_numbers_count + 1) {
                let entry: &mut u32 = cards_count_map.entry(i).or_insert(1);
                *entry += card_count;
            }

            let score = u32::pow(2, matching_numbers_count as u32) / 2;
            sum += score;
        }

        for (card, card_count) in &cards_count_map {
            println!("card: {}, card_count: {}", card, card_count);
        }

        let cards_count = cards_count_map.values().fold(0, |acc, &value| acc + value);
        println!("part1: score {}", sum);
        println!("part2: cards_count {}", cards_count);
    }

    let end_time = Instant::now();
    let duration: Duration = end_time - start_time;
    let duration_ms = duration.as_millis();
    println!("Execution took: {} milliseconds", duration_ms);
}
