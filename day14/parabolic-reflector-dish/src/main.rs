use std::fs;

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

fn roll_north(platform: &mut Vec<Vec<char>>) {
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
}

fn calculate_load(platform: &Vec<Vec<char>>) -> usize {
    platform.iter().enumerate().fold(0, |acc, (index, row)| {
        acc + row.iter().filter(|c| **c == 'O').count() * (platform.len() - index)
    })
}

fn main() {
    let mut platform = parse_platform("input.txt");
    roll_north(&mut platform);

    println!(
        "The sum of the load caused by the rounded rocks is: {:?}",
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
    fn rolls_north() {
        let expected = vec![
            vec!['O', 'O', 'O', 'O', '.', '#', '.', 'O', '.', '.'],
            vec!['O', 'O', '.', '.', '#', '.', '.', '.', '.', '#'],
            vec!['O', 'O', '.', '.', 'O', '#', '#', '.', '.', 'O'],
            vec!['O', '.', '.', '#', '.', 'O', 'O', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '#', '.'],
            vec!['.', '.', '#', '.', '.', '.', '.', '#', '.', '#'],
            vec!['.', '.', 'O', '.', '.', '#', '.', 'O', '.', 'O'],
            vec!['.', '.', 'O', '.', '.', '.', '.', '.', '.', '.'],
            vec!['#', '.', '.', '.', '.', '#', '#', '#', '.', '.'],
            vec!['#', '.', '.', '.', '.', '#', '.', '.', '.', '.'],
        ];

        let mut platform = parse_platform("test.txt");
        roll_north(&mut platform);

        assert_eq!(expected, platform)
    }

    #[test]
    fn calculates_load() {
        let mut platform = parse_platform("test.txt");
        roll_north(&mut platform);

        assert_eq!(136usize, calculate_load(&platform))
    }
}
