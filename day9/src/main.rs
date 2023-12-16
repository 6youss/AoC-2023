use crate::puzzle_input::PUZZLE_INPUT;

mod puzzle_input;
fn main() {
    for line in PUZZLE_INPUT
        .lines()
        .map(|l| l.split_whitespace().map(|n| n.parse::<u32>().unwrap()))
    {
        let mut steps_array = line.clone().collect::<Vec<u32>>();
        println!("{:?}", steps_array);
        loop {
            for i in 1..steps_array.len() {
                steps_array[i - 1] = steps_array[i] - steps_array[i - 1];
            }
            steps_array.pop();
            println!("{:?}", steps_array);

            if steps_array.iter().all(|e| *e == 0) {
                break;
            }
        }
    }
}
