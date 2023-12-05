use core::panic;
// Day 5, part 1 - If You Give A Seed A Fertilizer
use std::fs;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct RangeMap {
    source: u32,
    destination: u32,
    range: u32
}

// Shorthand to construct a RangeMap from an ordered tuple
impl RangeMap {
    fn new(values: (u32, u32, u32)) -> Self {
        Self {
            source: values.0,
            destination: values.1,
            range: values.2
        }
    }
}

fn parse_seeds(s: &str) -> Vec<u32> {
    let mut seeds: Vec<u32> = Vec::new();

    match s.split_once(": ") {
        Some((a, b)) if a == "seeds" => {
            for value in b.split_whitespace() {
                seeds.push(value.parse::<u32>().expect("Seed values should all be numbers."))
            }

            seeds
        },
        _ => panic!("Not a valid seed string")
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
        humidity_location_map
    ];

    let mut map_iterator = 0;

    for line in s.trim().lines() {
        match line {
            // On an empty line, move to the next map
            l if l.len() == 0 => map_iterator += 1,
            // On a line beginning with a number, process the line into the current map
            l if l.starts_with(|c: char| c.is_ascii_digit()) => {
                let values = l.split(" ");

                let values: Vec<u32> = values.map(|val| val.parse::<u32>().unwrap()).collect();
                
                maps[map_iterator].push(RangeMap::new((values[0], values[1], values[2])));
            },
            _ => continue,
        }
    }

    maps
}

fn find_lowest_location_value(file: &str) -> u32 {
    // Read input
    let input = fs::read_to_string(file).expect("File not found.");
    let mut seeds;
    let mut maps;

    // Get the seeds and maps
    match input.split_once('\n') {
        Some((a, b)) => {
            seeds = parse_seeds(a);
            maps = parse_maps(b);
        },
        _ => panic!("Input was not formatted as expected - please check file contents and try again."),
    }

    // Sort the maps by the source input to speed up the search process
    for map in maps {
        map.sort()
    }

    let mut minimum_location_value = std::u32::MAX;

    for seed in seeds {
        let mut value = seed;
        for map in maps {
            'current_range_map_loop: for range_map in map {
                // Since the RangeMaps are sorted, we can match
                match range_map {
                    rm if (rm.source..rm.source + rm.range).contains(value) => value = rm.destination + (value - rm.source),
                    rm if rm.source > value => break 'current_range_map_loop,
                    _ => (),
                }
            }
        }
    }
}

fn main() {
    println!("The lowest location value is {:?}", find_lowest_location_value("input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_seeds_as_expected() {
        let expected = vec![79, 14, 55, 13];
        let test_input = fs::read_to_string("test.txt").unwrap();

        let seeds = test_input.lines().next().unwrap();

        assert_eq!(expected, parse_seeds(seeds))
    }

    #[test]
    fn parses_maps_as_expected() {
        let expected = vec![
            vec![RangeMap::new((50, 98, 2)), RangeMap::new((52, 50, 48))],
            vec![RangeMap::new((0, 15, 37)), RangeMap::new((37, 52, 2)), RangeMap::new((39, 0, 15))],
            vec![RangeMap::new((49, 53, 8)), RangeMap::new((0, 11, 42)), RangeMap::new((42, 0, 7)), RangeMap::new((57, 7, 4))],
            vec![RangeMap::new((88, 18, 7)), RangeMap::new((18, 25, 70))],
            vec![RangeMap::new((45, 77, 23)), RangeMap::new((81, 45, 19)), RangeMap::new((68, 64, 13))],
            vec![RangeMap::new((0, 69, 1)), RangeMap::new((1, 0, 69))],
            vec![RangeMap::new((60, 56, 37)), RangeMap::new((56, 93, 4))],
        ];
        let test_input = fs::read_to_string("test.txt").unwrap();
        let (_, lines) = test_input.split_once('\n').unwrap();

        assert_eq!(expected, parse_maps(lines))
    }

    #[test]
    fn lowest_value_returned() {
        assert_eq!(35, find_lowest_location_value("test.txt"))
    }
}