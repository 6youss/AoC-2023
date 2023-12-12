use puzzle_input::ALMANAC;

use std::ops::Range;

mod puzzle_input;

#[derive(Debug, PartialEq, Eq, Hash)]
struct CategoryMapper {
    source: Range<u64>,
    dest: Range<u64>,
}

fn main() {
    let almanac_lines = ALMANAC.split("\n").filter(|&line| !line.trim().is_empty());

    let mut seeds_ranges: Vec<Range<u64>> = Vec::new();
    let mut currently_mapping = "";

    let mut seed_mappers: Vec<CategoryMapper> = Vec::new();
    let mut soil_mappers: Vec<CategoryMapper> = Vec::new();
    let mut fertilizer_mappers: Vec<CategoryMapper> = Vec::new();
    let mut water_mappers: Vec<CategoryMapper> = Vec::new();
    let mut light_mappers: Vec<CategoryMapper> = Vec::new();
    let mut temperature_mappers: Vec<CategoryMapper> = Vec::new();
    let mut humidity_mappers: Vec<CategoryMapper> = Vec::new();

    for line in almanac_lines {
        if line.contains("seeds:") {
            seeds_ranges = line
                .replace("seeds:", "")
                .split_whitespace()
                .map(|num_str| num_str.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
                .chunks(2)
                .map(|chunk| chunk[0]..chunk[0] + chunk[1])
                .collect();
        }
        let is_map_title_line = !line.chars().nth(0).unwrap().is_digit(10);
        if is_map_title_line {
            currently_mapping = line;
        } else {
            let map_values: Vec<u64> = line.split(" ").map(|n| n.parse::<u64>().unwrap()).collect();
            let dest = map_values[0];
            let source = map_values[1];
            let range_length = map_values[2];
            let source_range = source..source + range_length;
            let dest_range = dest..dest + range_length;
            if currently_mapping.contains("seed-to-soil") {
                seed_mappers.push(CategoryMapper {
                    source: source_range,
                    dest: dest_range,
                });
            } else if currently_mapping.contains("soil-to-fertilizer") {
                soil_mappers.push(CategoryMapper {
                    source: source_range,
                    dest: dest_range,
                });
            } else if currently_mapping.contains("fertilizer-to-water") {
                fertilizer_mappers.push(CategoryMapper {
                    source: source_range,
                    dest: dest_range,
                });
            } else if currently_mapping.contains("water-to-light") {
                water_mappers.push(CategoryMapper {
                    source: source_range,
                    dest: dest_range,
                });
            } else if currently_mapping.contains("light-to-temperature") {
                light_mappers.push(CategoryMapper {
                    source: source_range,
                    dest: dest_range,
                });
            } else if currently_mapping.contains("temperature-to-humidity") {
                temperature_mappers.push(CategoryMapper {
                    source: source_range,
                    dest: dest_range,
                });
            } else if currently_mapping.contains("humidity-to-location") {
                humidity_mappers.push(CategoryMapper {
                    source: source_range,
                    dest: dest_range,
                });
            }
        }
    }

    let soil_ranges = map_ranges(&seeds_ranges, seed_mappers);
    let fertilizer_ranges = map_ranges(&soil_ranges, soil_mappers);
    let water_ranges = map_ranges(&fertilizer_ranges, fertilizer_mappers);
    let light_ranges = map_ranges(&water_ranges, water_mappers);
    let temperature_ranges = map_ranges(&light_ranges, light_mappers);
    let humidity_ranges = map_ranges(&temperature_ranges, temperature_mappers);
    let location_ranges = map_ranges(&humidity_ranges, humidity_mappers);

    let min_location = location_ranges
        .iter()
        .map(|range| range.start)
        .min()
        .unwrap();
    println!("~~~~~~> minimun location {}", min_location);
}

fn map_ranges(ranges: &Vec<Range<u64>>, mappers: Vec<CategoryMapper>) -> Vec<Range<u64>> {
    let mut result: Vec<Range<u64>> = Vec::new();
    let mut ranges_stack = ranges.clone();

    while !ranges_stack.is_empty() {
        let range = ranges_stack.pop().unwrap();
        let mut has_matched_mapper = false;
        for mapper in &mappers {
            let range_mapping_results = map_range(&range, &mapper);
            let range_mapping_results_count = range_mapping_results.len();

            for rmr in range_mapping_results {
                if rmr.matched_mapper {
                    result.push(rmr.range);
                    has_matched_mapper = true;

                // else: only push to stack the case where one new range was created (the mapper created new unmatched ranges)
                } else if range_mapping_results_count > 1 {
                    ranges_stack.push(rmr.range);
                }
            }

            if has_matched_mapper == true {
                break;
            }
        }
        // if we checked all mappers and didnt hit any mapper for the current range
        // push it to resluts and go next
        if !has_matched_mapper {
            result.push(range);
        }
    }
    return result;
}

#[derive(Debug, PartialEq)]
enum RangeRelation {
    FullyInside,
    OutsideOnRight,
    OutsideOnLeft,
    PartiallyIntersectsOnLeft,
    PartiallyIntersectsOnRight,
    Superset,
}

fn determine_range_relation(reference: &Range<u64>, range: &Range<u64>) -> RangeRelation {
    if reference.start <= range.start && reference.end >= range.end {
        RangeRelation::FullyInside
    } else if range.start >= reference.end {
        RangeRelation::OutsideOnRight
    } else if reference.start >= range.end {
        RangeRelation::OutsideOnLeft
    } else if range.start <= reference.start && reference.end >= range.end {
        RangeRelation::PartiallyIntersectsOnLeft
    } else if reference.start <= range.start && reference.end <= range.end {
        RangeRelation::PartiallyIntersectsOnRight
    } else if reference.start > range.start && reference.end < range.end {
        RangeRelation::Superset
    } else {
        RangeRelation::FullyInside
    }
}

#[derive(PartialEq, Eq, Debug)]
struct RangeMapingResult {
    range: Range<u64>,
    matched_mapper: bool,
}

fn map_range(source_range: &Range<u64>, mapper: &CategoryMapper) -> Vec<RangeMapingResult> {
    let mut result: Vec<RangeMapingResult> = Vec::new();

    let range_relation = determine_range_relation(&mapper.source, source_range);

    match range_relation {
        RangeRelation::FullyInside => {
            result.push(RangeMapingResult {
                range: map_value_inside_range(source_range.start, &mapper)
                    ..map_value_inside_range(source_range.end, &mapper),
                matched_mapper: true,
            });
        }
        RangeRelation::PartiallyIntersectsOnRight => {
            result.push(RangeMapingResult {
                range: map_value_inside_range(source_range.start, &mapper)
                    ..map_value_inside_range(mapper.source.end, &mapper),
                matched_mapper: true,
            });
            result.push(RangeMapingResult {
                matched_mapper: false,
                range: mapper.source.end..source_range.end,
            });
        }
        RangeRelation::PartiallyIntersectsOnLeft => {
            result.push(RangeMapingResult {
                range: source_range.start..mapper.source.start,
                matched_mapper: false,
            });
            result.push(RangeMapingResult {
                range: map_value_inside_range(mapper.source.start, &mapper)
                    ..map_value_inside_range(source_range.end, &mapper),
                matched_mapper: true,
            });
        }
        RangeRelation::Superset => {
            result.push(RangeMapingResult {
                range: source_range.start..mapper.source.start,
                matched_mapper: false,
            });
            result.push(RangeMapingResult {
                range: map_value_inside_range(mapper.source.start, &mapper)
                    ..map_value_inside_range(mapper.source.end, &mapper),
                matched_mapper: true,
            });
            result.push(RangeMapingResult {
                range: mapper.source.end..source_range.end,
                matched_mapper: false,
            });
        }
        RangeRelation::OutsideOnLeft | RangeRelation::OutsideOnRight => {
            result.push(RangeMapingResult {
                range: source_range.clone(),
                matched_mapper: false,
            });
        }
    }

    return result;
}

fn map_value_inside_range(value: u64, mapper: &CategoryMapper) -> u64 {
    let diff: u64 = value - mapper.source.start;
    mapper.dest.start + diff
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_ranges() {
        assert_eq!(
            map_ranges(
                &vec![4..7],
                vec![
                    CategoryMapper {
                        source: 0..5,
                        dest: 50..55,
                    },
                    CategoryMapper {
                        source: 5..8,
                        dest: 100..103,
                    }
                ]
            ),
            vec![54..55, 100..102,]
        );
    }
    #[test]
    fn test_map_range() {
        assert_eq!(
            map_range(
                &(1..2),
                &CategoryMapper {
                    source: 0..5,
                    dest: 50..55,
                }
            ),
            vec![RangeMapingResult {
                range: 51..52,
                matched_mapper: true
            }]
        );
        assert_eq!(
            map_range(
                &(5..10),
                &CategoryMapper {
                    source: 0..5,
                    dest: 50..55,
                }
            ),
            vec![RangeMapingResult {
                range: 5..10,
                matched_mapper: false
            }]
        );
        assert_eq!(
            map_range(
                &(0..6),
                &CategoryMapper {
                    source: 1..5,
                    dest: 51..55,
                }
            ),
            vec![
                RangeMapingResult {
                    range: 0..1,
                    matched_mapper: false
                },
                RangeMapingResult {
                    range: 51..55,
                    matched_mapper: true
                },
                RangeMapingResult {
                    range: 5..6,
                    matched_mapper: false
                }
            ]
        );
        assert_eq!(
            map_range(
                &(2..6),
                &CategoryMapper {
                    source: 1..5,
                    dest: 51..55,
                }
            ),
            vec![
                RangeMapingResult {
                    range: 52..55,
                    matched_mapper: true
                },
                RangeMapingResult {
                    range: 5..6,
                    matched_mapper: false
                },
            ]
        );
        assert_eq!(
            map_range(
                &(0..4),
                &CategoryMapper {
                    source: 1..5,
                    dest: 51..55,
                }
            ),
            vec![
                RangeMapingResult {
                    range: 0..1,
                    matched_mapper: false
                },
                RangeMapingResult {
                    range: 51..54,
                    matched_mapper: true
                },
            ]
        );
    }

    #[test]
    fn test_map_value_inside_range() {
        assert_eq!(
            map_value_inside_range(
                5,
                &CategoryMapper {
                    source: 0..5,
                    dest: 50..55,
                }
            ),
            55
        );
        assert_eq!(
            map_value_inside_range(
                3,
                &CategoryMapper {
                    source: 0..5,
                    dest: 50..55,
                }
            ),
            53
        );
    }

    #[test]
    fn test_determine_range_relation() {
        assert_eq!(
            determine_range_relation(&(1..5), &(2..4)),
            RangeRelation::FullyInside
        );
        assert_eq!(
            determine_range_relation(&(1..2), &(1..2)),
            RangeRelation::FullyInside
        );
        assert_eq!(
            determine_range_relation(&(1..2), &(1..3)),
            RangeRelation::PartiallyIntersectsOnRight
        );
        assert_eq!(
            determine_range_relation(&(1..2), &(0..2)),
            RangeRelation::PartiallyIntersectsOnLeft
        );
        assert_eq!(
            determine_range_relation(&(1..2), &(0..3)),
            RangeRelation::Superset
        );
        assert_eq!(
            determine_range_relation(&(1..2), &(2..3)),
            RangeRelation::OutsideOnRight
        );
        assert_eq!(
            determine_range_relation(&(1..2), &(0..1)),
            RangeRelation::OutsideOnLeft
        );
    }
}
