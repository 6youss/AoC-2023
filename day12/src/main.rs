use puzzle_input::PUZZLE_INPUT;

mod puzzle_input;
fn main() {
    let mut total_possibilities = 0;
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

        let line_possibilities = possibilities(spring_arrangement, &groups);
        dbg!(spring_arrangement, line_possibilities);
        total_possibilities += line_possibilities;
        println!("total possibilities: {} ⭐️", total_possibilities);
    }
}

fn possibilities(springs: &str, groups: &Vec<usize>) -> usize {
    // println!("_____________________________________________");
    // dbg!(springs, groups);

    let springs = springs.trim_start_matches(".");
    if groups.len() == 0 {
        if springs.contains('#') {
            // dbg!("no groups left, but still some springs");
            return 0;
        }
        // dbg!("found one");
        return 1;
    }

    if springs.len() == 0 || springs.len() < groups.iter().sum::<usize>() {
        // dbg!("no springs left, but still some groups");
        return 0;
    }

    if springs.starts_with("#") {
        let group_length = groups[0];

        let chunk = &springs[..group_length];

        if chunk.contains(".") {
            // dbg!("seperator in group");
            return 0;
        }

        let next_is_seperator = match springs.chars().nth(group_length) {
            Some(c) => c != '#',
            None => true,
        };

        if !next_is_seperator {
            // dbg!("next is not seperator");
            return 0;
        }
        let hh = if groups.len() > 1 { 1 } else { 0 };
        return possibilities(&springs[group_length + hh..], &groups[1..].to_vec());
    };

    if springs.starts_with("?") {
        return possibilities(format!(".{}", &springs[1..]).as_str(), groups)
            + possibilities(format!("#{}", &springs[1..]).as_str(), groups);
    };
    // dbg!("no match");
    return 0;
}
