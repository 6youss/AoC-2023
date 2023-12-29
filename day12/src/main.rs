use puzzle_input::PUZZLE_INPUT;

mod puzzle_input;
fn main() {
    for line in PUZZLE_INPUT.lines() {
        let (spring_arrangement, groups) = {
            let line: Vec<&str> = line.split(" ").collect();
            (
                line[0],
                line[1]
                    .split(",")
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
            )
        };

        let is_accepted = is_accepted_line(spring_arrangement, &groups);
        dbg!(spring_arrangement, is_accepted);
    }
}

fn is_accepted_line(spring_arrangement: &str, groups: &Vec<usize>) -> bool {
    let mut new_line = spring_arrangement.trim_start_matches(".");

    if new_line.len() == 0 && groups.len() == 0 {
        return true;
    }
    if new_line.starts_with("#") {
        let group_end_length = groups[0];
        let chunk = &new_line[..group_end_length];

        if chunk.contains(".") {
            return false;
        }
        let next_char = new_line.chars().nth(group_end_length);

        let is_group_end = match next_char {
            Some(c) => c == '.',
            None => true,
        };
        if !is_group_end {
            return false;
        }
        new_line = &new_line[group_end_length..];
        return is_accepted_line(new_line, &groups[1..].to_vec());
    }
    return false;
}
