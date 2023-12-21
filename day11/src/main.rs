use std::collections::HashSet;

use puzzle_input::PUZZLE_INPUT;

mod puzzle_input;

use std::collections::VecDeque;

fn main() {
    let lines: Vec<&str> = PUZZLE_INPUT.lines().collect();
    let num_rows = lines.len();
    let num_cols: usize = lines[0].chars().count();
    let mut galaxis_positions: HashSet<(usize, usize, u32)> = HashSet::new();

    let mut galaxis_rows: HashSet<usize> = HashSet::new();
    let mut galaxis_cols: HashSet<usize> = HashSet::new();
    let mut galaxy_num = 0;
    for row in 0..num_rows {
        for col in 0..num_cols {
            if lines[row].chars().nth(col).unwrap() == '#' {
                galaxis_rows.insert(row);
                galaxis_cols.insert(col);
                galaxy_num += 1;
                galaxis_positions.insert((row, col, galaxy_num));
            }
        }
    }

    let possible_pairs = find_pairs(&galaxis_positions.into_iter().collect());
    let sum: u128 = possible_pairs
        .iter()
        .map(|pair| {
            let (start_x, start_y, _) = pair.0;
            let (end_x, end_y, _) = pair.1;

            let shortest_distance = find_closest_road(
                &lines,
                (start_x, start_y),
                (end_x, end_y),
                &galaxis_rows,
                &galaxis_cols,
            );
            // println!("{:?} => {:?} = {}", start_num, end_num, shortest_distance);
            shortest_distance as u128
        })
        .sum();

    println!("⭐️⭐️ {}", sum);
}

fn find_pairs<T: Clone>(arr: &Vec<T>) -> Vec<(T, T)> {
    let mut pairs = Vec::new();

    for i in 0..arr.len() {
        for j in i + 1..arr.len() {
            pairs.push((arr[i].clone(), arr[j].clone()));
        }
    }

    pairs
}

fn is_valid_move(lines: &Vec<&str>, visited: &HashSet<(usize, usize)>, row: i32, col: i32) -> bool {
    let rows = lines.len() as usize;
    let cols = lines[0].len() as usize;
    row >= 0
        && (row as usize) < rows
        && col >= 0
        && (col as usize) < cols
        && !visited.contains(&(row as usize, col as usize))
}

fn find_closest_road(
    lines: &Vec<&str>,
    start: (usize, usize),
    end: (usize, usize),
    galaxies_rows: &HashSet<usize>,
    galaxies_cols: &HashSet<usize>,
) -> i32 {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    visited.insert(start);

    let dr: i32 = (end.0 as i32) - (start.0 as i32);
    let dc = (end.1 as i32) - (start.1 as i32);
    let mut moves = Vec::new();
    if dr != 0 {
        moves.push((dr / dr.abs(), 0));
    }
    if dc != 0 {
        moves.push((0, dc / dc.abs()));
    }
    while let Some(((row, col), distance)) = queue.pop_front() {
        if (row, col) == end {
            return distance;
        }

        for (row_move, col_move) in moves.iter() {
            let new_row: i32 = (row as i32) + row_move;
            let new_col: i32 = (col as i32) + col_move;

            if is_valid_move(&lines, &visited, new_row, new_col) {
                visited.insert((new_row as usize, new_col as usize));
                let expansion_factor =
                    if *row_move != 0 && !galaxies_rows.contains(&(new_row as usize)) {
                        1000000
                    } else if *col_move != 0 && !galaxies_cols.contains(&(new_col as usize)) {
                        1000000
                    } else {
                        1
                    };
                queue.push_back((
                    (new_row as usize, new_col as usize),
                    distance + expansion_factor,
                ));
            }
        }
    }

    -1
}
