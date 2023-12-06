// Day 5, part 2 - If You Give A Seed A Fertilizer
use std::fs;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct RangeMap {
    source: u64,
    destination: u64,
    range: u64,
}

// Shorthand to construct a RangeMap from an ordered tuple
impl RangeMap {
    fn new(values: (u64, u64, u64)) -> Self {
        Self {
            source: values.1,
            destination: values.0,
            range: values.2,
        }
    }
}

fn parse_seeds(s: &str) -> Vec<RangeMap> {
    let mut values: Vec<u64> = Vec::new();
    let mut seeds: Vec<RangeMap> = Vec::new();

    match s.split_once(": ") {
        Some((a, b)) if a == "seeds" => {
            for value in b.split_whitespace() {
                values.push(
                    value
                        .parse::<u64>()
                        .expect("Seed values should all be numbers."),
                )
            }

            let mut previous_value = 0;

            for (index, value) in values.into_iter().enumerate() {
                match index {
                    i if i % 2 == 0 => previous_value = value,
                    _ => {
                        seeds.push(RangeMap::new((
                            value + previous_value,
                            previous_value,
                            value,
                        )));
                    }
                }
            }

            seeds
        }
        _ => panic!("Not a valid seed string"),
    }
}

fn parse_maps(s: &str) -> Vec<Vec<RangeMap>> {
    // Create maps
    let seeds_soil_map: Vec<RangeMap> = Vec::new();
    let soil_fertilizer_map: Vec<RangeMap> = Vec::new();
    let fertilizer_water_map: Vec<RangeMap> = Vec::new();
    let water_light_map: Vec<RangeMap> = Vec::new();
    let light_temperature_map: Vec<RangeMap> = Vec::new();
    let temperature_humidity_map: Vec<RangeMap> = Vec::new();
    let humidity_location_map: Vec<RangeMap> = Vec::new();

    // Aggregate maps
    let mut maps = vec![
        seeds_soil_map,
        soil_fertilizer_map,
        fertilizer_water_map,
        water_light_map,
        light_temperature_map,
        temperature_humidity_map,
        humidity_location_map,
    ];

    let mut map_iterator = 0;

    for line in s.trim().lines() {
        match line {
            // On an empty line, move to the next map
            l if l.len() == 0 => map_iterator += 1,
            // On a line beginning with a number, process the line into the current map
            l if l.starts_with(|c: char| c.is_ascii_digit()) => {
                let values = l.split(" ");

                let values: Vec<u64> = values.map(|val| val.parse::<u64>().unwrap()).collect();

                maps[map_iterator].push(RangeMap::new((values[0], values[1], values[2])));
            }
            _ => continue,
        }
    }

    maps
}

fn find_lowest_location_value(file: &str) -> u64 {
    // Read input
    let input = fs::read_to_string(file).expect("File not found.");
    let seeds;
    let mut maps;

    // Get the seeds and maps
    match input.split_once('\n') {
        Some((a, b)) => {
            seeds = parse_seeds(a);
            maps = parse_maps(b);
        }
        _ => panic!(
            "Input was not formatted as expected - please check file contents and try again."
        ),
    }

    // Sort the maps by the source input to speed up the search process
    for map in &mut maps {
        map.sort()
    }

    let mut minimum_location_value = std::u64::MAX;

    for seed in seeds {
        for seed in seed.source..seed.destination {
            let mut value = seed;
            for map in &maps {
                'current_range_map_loop: for range_map in map {
                    // Since the RangeMaps are sorted, we can break out early if we
                    // a) find a match for our target range, or
                    // b) find a source greater than our value
                    match range_map {
                        rm if (rm.source..rm.source + rm.range).contains(&value) => {
                            value = rm.destination + (value - rm.source);
                            break 'current_range_map_loop;
                        }
                        rm if rm.source > value => break 'current_range_map_loop,
                        _ => (),
                    }
                }
            }
            minimum_location_value = std::cmp::min(minimum_location_value, value);
        }
    }

    minimum_location_value
}

fn main() {
    println!(
        "The lowest location value is {:?}",
        find_lowest_location_value("input.txt")
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_seeds_as_expected() {
        let expected: Vec<RangeMap> =
            vec![RangeMap::new((93, 79, 14)), RangeMap::new((68, 55, 13))];

        let test_input = fs::read_to_string("test.txt").unwrap();

        let seeds = test_input.lines().next().unwrap();

        assert_eq!(expected, parse_seeds(seeds))
    }

    #[test]
    fn parses_maps_as_expected() {
        let expected = vec![
            vec![RangeMap::new((50, 98, 2)), RangeMap::new((52, 50, 48))],
            vec![
                RangeMap::new((0, 15, 37)),
                RangeMap::new((37, 52, 2)),
                RangeMap::new((39, 0, 15)),
            ],
            vec![
                RangeMap::new((49, 53, 8)),
                RangeMap::new((0, 11, 42)),
                RangeMap::new((42, 0, 7)),
                RangeMap::new((57, 7, 4)),
            ],
            vec![RangeMap::new((88, 18, 7)), RangeMap::new((18, 25, 70))],
            vec![
                RangeMap::new((45, 77, 23)),
                RangeMap::new((81, 45, 19)),
                RangeMap::new((68, 64, 13)),
            ],
            vec![RangeMap::new((0, 69, 1)), RangeMap::new((1, 0, 69))],
            vec![RangeMap::new((60, 56, 37)), RangeMap::new((56, 93, 4))],
        ];
        let test_input = fs::read_to_string("test.txt").unwrap();
        let (_, lines) = test_input.split_once('\n').unwrap();

        assert_eq!(expected, parse_maps(lines))
    }

    #[test]
    fn lowest_value_returned() {
        assert_eq!(46, find_lowest_location_value("test.txt"))
    }
}
