use std::collections::HashMap;

fn main() {
    let input = "Time:        55     82     64     90
    Distance:   246   1441   1012   1111";

    let mut time_distance_map: HashMap<u64, u64> = HashMap::new();
    let lines: Vec<&str> = input.lines().collect();
    let times: Vec<u64> = lines[0]
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .split(":")
        .filter_map(|s| s.parse().ok())
        .collect();

    let distances: Vec<u64> = lines[1]
        .split_whitespace()
        .collect::<String>()
        .split(":")
        .filter_map(|s| s.parse().ok())
        .collect();

    for (time, distance) in times.into_iter().zip(distances.into_iter()) {
        time_distance_map.insert(time, distance);
    }

    let mut breaking_records_map: HashMap<u64, u64> = HashMap::new();

    for (time, distance) in &time_distance_map {
        let mut breaking_record_count = breaking_records_map.get(&time).unwrap_or(&0).clone();
        for i in 0..=*time {
            if distance < &get_distance_traveled(0, i, *time) {
                breaking_record_count += 1;
            }
        }
        breaking_records_map.insert(*time, breaking_record_count);
    }
    let product: u64 = breaking_records_map.values().cloned().product();
    println!("time_distance_map {:?}", time_distance_map);
    println!("get_distance_traveled {}", product);
}

fn get_distance_traveled(init_speed: u64, consumed_time: u64, total_time: u64) -> u64 {
    let speed_accelaration = 1;
    let speed = consumed_time;
    (speed + init_speed) * speed_accelaration * (total_time - consumed_time)
}
