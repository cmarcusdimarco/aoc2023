use std::{collections::HashMap, fs};

fn parse_platform(path: &str) -> Vec<Vec<char>> {
    let input = fs::read_to_string(path).expect("Error reading file at specified path");
    let mut platform: Vec<Vec<char>> = Vec::new();

    for (index, line) in input.lines().enumerate() {
        platform.push(Vec::new());

        for c in line.chars() {
            platform[index].push(c);
        }
    }

    platform
}

// Inspired by the solution from HyperNeutrino in Python,
// we'll shift our function below to be "rotate and tilt"
// instead of rolling in a direction.
// We'll also call this function until we see a repetition, at
// which point we will check for the number of iterations that
// occurred between repetitions and determine the final position
// using the modulo of the remaining cycles % cycles between repetition.
fn tilt_and_rotate(platform: &mut Vec<Vec<char>>) {
    // We'll take an approach of iterating through the columns,
    // starting from the final row. Sum the Os (round rocks) encountered
    // until hitting a # (cube rock), then replace the chars from rows
    // n + 1 through x - 1, where x is the index of the last # seen (or the
    // max value, if a # hasn't been seen yet.)
    for column in 0..platform[0].len() {
        let mut counter = 0;
        let mut previous_cube = platform.len();

        for (i, row) in platform.clone().iter().enumerate().rev() {
            match row[column] {
                'O' => counter += 1,
                '#' => {
                    for index in i + 1..previous_cube {
                        match counter {
                            x if x > 0 => {
                                platform[index][column] = 'O';
                                counter -= 1;
                            }
                            _ => platform[index][column] = '.',
                        }
                    }

                    previous_cube = i;
                }
                _ => (),
            }
        }

        // If we have reached this point with a non-zero counter, we need to push to the top.
        if counter > 0 {
            for index in 0..previous_cube {
                match counter {
                    x if x > 0 => {
                        platform[index][column] = 'O';
                        counter -= 1;
                    }
                    _ => platform[index][column] = '.',
                }
            }
        }
    }

    // Inspired by https://stackoverflow.com/questions/65505015/whats-the-best-way-to-switch-columns-and-rows-in-a-2d-array-in-rust
    let rotation: Vec<Vec<char>> = (0..platform[0].len())
        .map(|i| platform.iter().rev().map(|x| x[i]).collect())
        .collect();

    platform.clone_from(&rotation)
}

fn cycle(platform: &mut Vec<Vec<char>>) {
    for _ in 0..4 {
        tilt_and_rotate(platform);
    }
}

fn calculate_load(platform: &Vec<Vec<char>>) -> usize {
    platform.iter().enumerate().fold(0, |acc, (index, row)| {
        acc + row.iter().filter(|c| **c == 'O').count() * (platform.len() - index)
    })
}

fn main() {
    let mut platform = parse_platform("input.txt");
    let mut cache: HashMap<Vec<Vec<char>>, usize> = HashMap::new();

    let mut counter: usize = 0;

    // Loop until we hit a repeat pattern
    while let None = cache.get(&platform) {
        // Using a lot of memory for this, but brute forcing 1,000,000,000 loops isn't feasible.
        cache.insert(platform.clone(), counter);
        cycle(&mut platform);
        counter += 1;
    }

    // Once we hit the repetition, only cycle (1,000,000,000 - counter) % (counter - repetition_counter) times.
    let remainder = 1_000_000_000 - counter;
    let modulus = counter
        - cache
            .get(&platform)
            .expect("Retrieval should succeed since we just inserted into the map");

    for _ in 0..(remainder % modulus) {
        cycle(&mut platform);
    }

    println!(
        "The sum of the load caused by the rounded rocks after 1,000,000,000 cycles is: {:?}",
        calculate_load(&platform)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_platform() {
        let expected = vec![
            vec!['O', '.', '.', '.', '.', '#', '.', '.', '.', '.'],
            vec!['O', '.', 'O', 'O', '#', '.', '.', '.', '.', '#'],
            vec!['.', '.', '.', '.', '.', '#', '#', '.', '.', '.'],
            vec!['O', 'O', '.', '#', 'O', '.', '.', '.', '.', 'O'],
            vec!['.', 'O', '.', '.', '.', '.', '.', 'O', '#', '.'],
            vec!['O', '.', '#', '.', '.', 'O', '.', '#', '.', '#'],
            vec!['.', '.', 'O', '.', '.', '#', 'O', '.', '.', 'O'],
            vec!['.', '.', '.', '.', '.', '.', '.', 'O', '.', '.'],
            vec!['#', '.', '.', '.', '.', '#', '#', '#', '.', '.'],
            vec!['#', 'O', 'O', '.', '.', '#', '.', '.', '.', '.'],
        ];

        assert_eq!(expected, parse_platform("test.txt"))
    }

    #[test]
    fn cycles() {
        let expected = vec![
            vec!['.', '.', '.', '.', '.', '#', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '#', '.', '.', '.', 'O', '#'],
            vec!['.', '.', '.', 'O', 'O', '#', '#', '.', '.', '.'],
            vec!['.', 'O', 'O', '#', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', 'O', 'O', 'O', '#', '.'],
            vec!['.', 'O', '#', '.', '.', '.', 'O', '#', '.', '#'],
            vec!['.', '.', '.', '.', 'O', '#', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', 'O', 'O', 'O', 'O'],
            vec!['#', '.', '.', '.', 'O', '#', '#', '#', '.', '.'],
            vec!['#', '.', '.', 'O', 'O', '#', '.', '.', '.', '.'],
        ];

        let mut platform = parse_platform("test.txt");
        cycle(&mut platform);

        assert_eq!(expected, platform)
    }

    #[test]
    fn calculates_load() {
        let mut platform = parse_platform("test.txt");
        let mut cache: HashMap<Vec<Vec<char>>, usize> = HashMap::new();

        let mut counter: usize = 0;

        // Loop until we hit a repeat pattern
        while let None = cache.get(&platform) {
            // Using a lot of memory for this, but brute forcing 1,000,000,000 loops isn't feasible.
            cache.insert(platform.clone(), counter);
            cycle(&mut platform);
            counter += 1;
        }

        // Once we hit the repetition, only cycle (1,000,000,000 - counter) % (counter - repetition_counter) times.
        let remainder = 1_000_000_000 - counter;
        let modulus = counter
            - cache
                .get(&platform)
                .expect("Retrieval should succeed since we just inserted into the map");

        for _ in 0..(remainder % modulus) {
            cycle(&mut platform);
        }

        assert_eq!(64usize, calculate_load(&platform))
    }
}
