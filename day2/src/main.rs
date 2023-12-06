mod games_record;

use games_record::GAMES_RECORD;
fn main() {
    const RED: u32 = 12;
    const GREEN: u32 = 13;
    const BLUE: u32 = 14;
    let mut sum_of_possible_games = 0;
    // PART 2 (Comment part 1 and uncomment part 2 to get part 2 result)
    // let mut power_sum = 0;
    for game in GAMES_RECORD.split('\n') {
        if !game.contains("Game") {
            continue;
        }
        let splited_line = game.split(":").collect::<Vec<&str>>();

        let game_str = splited_line[0];
        let game_subsets = splited_line[1];
        let mut is_game_possible = true;

        // PART 2 (Comment part 1 and uncomment part 2 to get part 2 result)
        // let mut max_red = 0;
        // let mut max_green = 0;
        // let mut max_blue = 0;
        'outer: for game_subset in game_subsets.split(";") {
            for cube_set in game_subset.split(",") {
                let cube_number_color = cube_set.trim().split(" ").collect::<Vec<&str>>();
                let cubes_number = cube_number_color[0].parse::<u32>().unwrap();
                let cubes_color = cube_number_color[1];

                // PART 1
                if (cubes_color == "red" && cubes_number > RED)
                    || (cubes_color == "green" && cubes_number > GREEN)
                    || (cubes_color == "blue" && cubes_number > BLUE)
                {
                    is_game_possible = false;
                    break 'outer;
                }

                // PART 2 (Comment part 1 and uncomment part 2 to get part 2 result)
                // match cubes_color {
                //     "red" => {
                //         if cubes_number > max_red {
                //             max_red = cubes_number;
                //         }
                //     }
                //     "green" => {
                //         if cubes_number > max_green {
                //             max_green = cubes_number;
                //         }
                //     }
                //     "blue" => {
                //         if cubes_number > max_blue {
                //             max_blue = cubes_number;
                //         }
                //     }
                //     _ => {}
                // }
            }
        }
        if is_game_possible {
            let game_id = game_str.split(" ").collect::<Vec<&str>>()[1]
                .parse::<u32>()
                .unwrap();
            sum_of_possible_games += game_id;
        }
        // PART 2 (Comment part 1 and uncomment part 2 to get part 2 result)
        // power_sum += max_blue * max_green * max_red;
    }
    println!("Sum of possible games: {}", sum_of_possible_games);

    // PART 2 (Comment part 1 and uncomment part 2 to get part 2 result)
    // println!("Power sum: {}", power_sum);
}
