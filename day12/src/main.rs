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

        let possibilities = possibilities(spring_arrangement, &groups);
        dbg!(spring_arrangement, possibilities);
    }
}

fn possibilities(spring_arrangement: &str, groups: &Vec<usize>) -> usize {
    let mut new_line = spring_arrangement.trim_start_matches(".");

    if groups.len() == 0 {
        if new_line.len() == 0 {
            return 1;
        }
        return 0;
    }

    if new_line.len() < groups.iter().sum() {
        return 0;
    }

    if new_line.starts_with("#") {
        let group_value = groups[0];

        let chunk = &new_line[..group_value];

        if chunk.contains(".") {
            return 0;
        }
        let next_char = new_line.chars().nth(group_value);

        let next_is_seperator = match next_char {
            Some(c) => c != '#',
            None => true,
        };
        if !next_is_seperator {
            return 0;
        }
        new_line = &new_line[group_value..];
        return possibilities(new_line, &groups[1..].to_vec());
    };

    return possibilities(&new_line.replacen("?", "#", 1), groups)
        + possibilities(&new_line.replacen("?", ".", 1), groups);
}
