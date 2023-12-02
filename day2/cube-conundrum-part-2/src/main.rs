// Day 2, part 2: Cube Conundrum
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    // Read input
    let input = fs::read_to_string("input.txt")?;
    let mut powers: Vec<i32> = Vec::new();

    // For each game...
    for game in input.lines() {
        let mut red_max = 0;
        let mut green_max = 0;
        let mut blue_max = 0;
        let mut value = 0;

        // ...parse the line for violations of our thresholds.
        for section in game.split_whitespace() {
            match section {
                s if s.parse::<i32>().unwrap_or(0) != 0 => {
                    // Store the value so we can compare it conditionally based on the string that follows.
                    value = s.parse::<i32>().unwrap();
                }
                s if s.contains("red") => {
                    if value > red_max {
                        red_max = value;
                    }
                }
                s if s.contains("green") => {
                    if value > green_max {
                        green_max = value;
                    }
                }
                s if s.contains("blue") => {
                    if value > blue_max {
                        blue_max = value;
                    }
                }
                _ => (),
            }
        }

        powers.push(red_max * green_max * blue_max);
    }

    // Now that we've processed all lines, sum the elements in powers.
    let sum = powers
        .into_iter()
        .reduce(|a, b| a + b)
        .expect("There was an error calculating the sum of the vector elements.");

    println!("The sum of the powers is: {:?}", sum);

    Ok(())
}
