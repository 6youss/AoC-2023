use crate::puzzle_input::PUZZLE_INPUT;

mod puzzle_input;
fn main() {
    let mut sum = 0;
    for line in PUZZLE_INPUT
        .lines()
        .map(|l| l.split_whitespace().map(|n| n.parse::<i32>().unwrap()))
    {
        sum += get_next_value(&line.collect());
    }
    println!("⭐️ {}", sum);
}

fn get_next_value(array: &Vec<i32>) -> i32 {
    let mut steps_array: Vec<i32> = array.iter().cloned().collect();
    let mut lost_stack: Vec<i32> = Vec::new();
    println!("{:?}", steps_array);
    loop {
        for i in 1..steps_array.len() {
            steps_array[i - 1] = steps_array[i] - steps_array[i - 1];
        }
        let lost_element = steps_array.pop().unwrap();
        lost_stack.push(lost_element);
        println!("{:?}", steps_array);

        if steps_array.iter().all(|e| *e == 0) {
            return lost_stack.iter().sum();
        }
    }
}
