use std::fs;

#[derive(Debug, PartialEq, Eq)]
struct Race {
    time: u32,
    distance: u32
}

impl Race {
    fn empty() -> Self {
        Self {
            time: 0,
            distance: 0
        }
    }
}

fn parse_races(file_path: &str) -> Vec<Race> {
    let input = fs::read_to_string(file_path).expect("Error parsing the file at the specified path");

    let mut races: Vec<Race> = Vec::new();

    match input.split_once('\n') {
        Some((a, b)) if a.starts_with("Time:") && b.starts_with("Distance:") => {
            // Reduce the slices to just the data
            let a = a.split_once(':').unwrap().1.trim().replace(" ", "");
            let value = a.parse::<u32>().expect("There was an error parsing the value from the input");
            let mut race = Race::empty();
            race.time = value;

            let b = b.split_once(':').unwrap().1.trim().replace(" ", "");
            race.distance = b.parse::<u32>().unwrap();
            
            races.push(race);
        },
        _ => panic!("File did not match expected format. Please verify file contents and try again."),
    }

    races
}

fn quantify_winning_strategies(races: &Vec<Race>) -> Vec<u32> {
    // The furthest distance achievable in any race is achieved at race.time / 2.
    // The minimum winning distance achievable is at the value of x closest to 0 which satisfies
    //     x * (race.time - x) > race.distance
    // The total set of winning_strategies lies within the set of integers y that are bound by x < y < race.time - x
    // This quantity is equal to ((race.time / 2) - x + 1) * 2  where race.time is an odd number, and
    //     ((race.time / 2) - x + 1) * 2 - 1 where race.time is an even number
    
    let mut winning_strategies: Vec<u32> = Vec::new();

    for race in races {
        'current_race: for  x in 1..race.time / 2 {
            if x * (race.time - x) > race.distance {
                let quantity = match race.time {
                    t if t % 2 != 0 => ((race.time / 2) - x + 1) * 2,
                    _ => ((race.time / 2) - x + 1) * 2 - 1
                };

                winning_strategies.push(quantity);

                break 'current_race;
            }
        }
    }

    winning_strategies
}

fn main() {
    let races = parse_races("input.txt");

    let winning_strategies = quantify_winning_strategies(&races);

    let product: u32 = winning_strategies.iter().product();

    println!("The product of all winning strategies is: {:?}", product);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_races() {
        let expected: Vec<Race> = vec![Race {time: 71530, distance: 940200}];

        assert_eq!(expected, parse_races("test.txt"))
    }

    #[test]
    fn correct_winning_strategies() {
        let expected = vec![71503];

        assert_eq!(expected, quantify_winning_strategies(&parse_races("test.txt")))
    }
}