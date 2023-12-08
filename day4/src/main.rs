use puzzle_input::PUZZLE_INPUT;

mod puzzle_input;
fn main() {
    let mut sum = 0;
    for card in PUZZLE_INPUT.split('\n') {
        let table_row: Vec<&str> = card.split(":").collect::<Vec<&str>>();

        let card_title = table_row[0];

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
        println!(
            "{}: number_owned_winning:{} score:{}",
            card_title, number_owned_winning, score
        );
        sum += score;
    }
    println!("sum {}", sum);
}
