// Day 2, part 1: Cube Conundrum
use std::{collections::HashMap, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    // Read input
    let input = fs::read_to_string("input.txt")?;
    let mut possible_games: Vec<i32> = Vec::new();

    // Set thresholds for impossible games
    let thresholds = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    // For each game...
    'outer: for game in input.lines() {
        let mut id = 0;
        let mut value = 0;

        // ...parse the line for violations of our thresholds.
        for section in game.split_whitespace() {
            match section {
                // Our input guarantees that game IDs are followed by a ":", and that a ":" only appears after a game ID
                s if s.ends_with(":") => {
                    id = s
                        .trim_end_matches(":")
                        .parse::<i32>()
                        .expect("Error parsing game ID from section.")
                }
                s if s.parse::<i32>().unwrap_or(0) != 0 => {
                    // Store the value so we can compare it conditionally based on the string that follows.
                    value = s.parse::<i32>().unwrap();

                    // If any value exceeds the max threshold, we can stop processing the section and consider the game impossible.
                    if value > thresholds["blue"] {
                        continue 'outer;
                    }
                }
                s if s.contains("red") => {
                    if value > thresholds["red"] {
                        continue 'outer;
                    }
                }
                s if s.contains("green") => {
                    if value > thresholds["green"] {
                        continue 'outer;
                    }
                }
                s if s.contains("blue") => {
                    if value > thresholds["blue"] {
                        continue 'outer;
                    }
                }
                _ => (),
            }
        }

        if id != 0 {
            possible_games.push(id);
        }
    }

    // Now that we've processed all lines, sum the elements in possible_games.
    let sum = possible_games
        .into_iter()
        .reduce(|a, b| a + b)
        .expect("There was an error calculating the sum of the vector elements.");

    println!("The sum of the IDs of possible games is: {:?}", sum);

    Ok(())
}
