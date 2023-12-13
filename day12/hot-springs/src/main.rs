use std::fs;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug, PartialEq, Eq)]
struct Row {
    springs: Vec<Condition>,
    sequence: Vec<u32>,
}

fn parse_rows(path: &str) -> Vec<Row> {
    let input = fs::read_to_string(path).expect("Error reading file at specified path");
    let mut rows: Vec<Row> = Vec::new();

    for line in input.lines() {
        let mut row = Row {
            springs: Vec::new(),
            sequence: Vec::new(),
        };

        let (condition_records, damaged_spring_sequence) = line
            .split_once(' ')
            .expect("Each line should be separated by a single space.");

        for record in condition_records.chars() {
            let condition = match record {
                '.' => Condition::Operational,
                '#' => Condition::Damaged,
                '?' => Condition::Unknown,
                _ => panic!("Unexpected character present in condition_records"),
            };

            row.springs.push(condition);
        }

        for segment in damaged_spring_sequence.split(',') {
            let segment = segment
                .parse::<u32>()
                .expect("Non-numeric value present in damaged_spring_sequence");

            row.sequence.push(segment);
        }

        rows.push(row);
    }

    rows
}

fn calculate_possible_arrangements(row: &Row) -> u32 {
    // Our strategy will be to recurse through valid combinations and break out early
    // if there is a conflict between the combination and the Row's sequence.

    // Start the recursion
    recurse(&row, 0, Vec::new())
}

fn recurse(row: &Row, index: usize, mut accumulator: Vec<Condition>) -> u32 {
    // Base case
    if index == row.springs.len() {
        return match sequence_is_valid(&accumulator, &row.sequence) {
            true => 1u32,
            false => 0u32,
        };
    } else {
        let current_spring = row.springs.iter().nth(index);

        return match current_spring {
            Some(Condition::Unknown) => {
                let mut second_accumulator = accumulator.clone();
                accumulator.push(Condition::Damaged);
                second_accumulator.push(Condition::Operational);
                recurse(row, index + 1, accumulator) + recurse(row, index + 1, second_accumulator)
            }
            Some(c) => {
                accumulator.push(c.clone());
                recurse(row, index + 1, accumulator)
            }
            None => panic!("Index violation on row.springs"),
        };
    }
}

fn sequence_is_valid(springs: &Vec<Condition>, target_sequence: &Vec<u32>) -> bool {
    // There are 2^n possible combinations, where n is the number of Condition::Unknowns.
    //
    // We can reduce this number firstly by calculating the difference between the sum
    // of a Row's sequence and the amount of Condition::Damaged present.
    let accumulated_damaged = springs.iter().fold(0u32, |acc, x| {
        if let Condition::Damaged = x {
            acc + 1
        } else {
            acc
        }
    });

    if accumulated_damaged != target_sequence.iter().sum() {
        return false;
    }

    let mut sequence: Vec<u32> = Vec::new();
    let mut counter = 0;

    for condition in springs.iter() {
        match condition {
            Condition::Damaged => counter += 1,
            Condition::Operational => {
                if counter > 0 {
                    sequence.push(counter);
                }
                counter = 0;
            }
            _ => (),
        }
    }

    if counter > 0 {
        sequence.push(counter)
    }

    *target_sequence == sequence
}

fn main() {
    let rows = parse_rows("input.txt");
    let sum = rows
        .iter()
        .fold(0u32, |acc, x| acc + calculate_possible_arrangements(x));

    println!(
        "The sum of possible arrangements of broken equipment is: {:?}",
        sum
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_rows() {
        let expected = vec![
            Row {
                springs: vec![
                    Condition::Unknown,
                    Condition::Unknown,
                    Condition::Unknown,
                    Condition::Operational,
                    Condition::Damaged,
                    Condition::Damaged,
                    Condition::Damaged,
                ],
                sequence: vec![1, 1, 3],
            },
            Row {
                springs: vec![
                    Condition::Operational,
                    Condition::Unknown,
                    Condition::Unknown,
                    Condition::Operational,
                    Condition::Operational,
                    Condition::Unknown,
                    Condition::Unknown,
                    Condition::Operational,
                    Condition::Operational,
                    Condition::Operational,
                    Condition::Unknown,
                    Condition::Damaged,
                    Condition::Damaged,
                    Condition::Operational,
                ],
                sequence: vec![1, 1, 3],
            },
            Row {
                springs: vec![
                    Condition::Unknown,
                    Condition::Damaged,
                    Condition::Unknown,
                    Condition::Damaged,
                    Condition::Unknown,
                    Condition::Damaged,
                    Condition::Unknown,
                    Condition::Damaged,
                    Condition::Unknown,
                    Condition::Damaged,
                    Condition::Unknown,
                    Condition::Damaged,
                    Condition::Unknown,
                    Condition::Damaged,
                    Condition::Unknown,
                ],
                sequence: vec![1, 3, 1, 6],
            },
            Row {
                springs: vec![
                    Condition::Unknown,
                    Condition::Unknown,
                    Condition::Unknown,
                    Condition::Unknown,
                    Condition::Operational,
                    Condition::Damaged,
                    Condition::Operational,
                    Condition::Operational,
                    Condition::Operational,
                    Condition::Damaged,
                    Condition::Operational,
                    Condition::Operational,
                    Condition::Operational,
                ],
                sequence: vec![4, 1, 1],
            },
            Row {
                springs: vec![
                    Condition::Unknown,
                    Condition::Unknown,
                    Condition::Unknown,
                    Condition::Unknown,
                    Condition::Operational,
                    Condition::Damaged,
                    Condition::Damaged,
                    Condition::Damaged,
                    Condition::Damaged,
                    Condition::Damaged,
                    Condition::Damaged,
                    Condition::Operational,
                    Condition::Operational,
                    Condition::Damaged,
                    Condition::Damaged,
                    Condition::Damaged,
                    Condition::Damaged,
                    Condition::Damaged,
                    Condition::Operational,
                ],
                sequence: vec![1, 6, 5],
            },
            Row {
                springs: vec![
                    Condition::Unknown,
                    Condition::Damaged,
                    Condition::Damaged,
                    Condition::Damaged,
                    Condition::Unknown,
                    Condition::Unknown,
                    Condition::Unknown,
                    Condition::Unknown,
                    Condition::Unknown,
                    Condition::Unknown,
                    Condition::Unknown,
                    Condition::Unknown,
                ],
                sequence: vec![3, 2, 1],
            },
        ];

        assert_eq!(expected, parse_rows("test.txt"))
    }

    #[test]
    fn calculates_possible_arrangements() {
        let rows = parse_rows("test.txt");

        let actual = rows
            .iter()
            .fold(0u32, |acc, x| acc + calculate_possible_arrangements(x));

        assert_eq!(21, actual)
    }

    #[test]
    fn validates_sequence() {
        let row = Row {
            springs: vec![
                Condition::Damaged,
                Condition::Operational,
                Condition::Damaged,
                Condition::Operational,
                Condition::Damaged,
                Condition::Damaged,
                Condition::Damaged,
            ],
            sequence: vec![1, 1, 3],
        };

        assert_eq!(true, sequence_is_valid(&row.springs, &row.sequence))
    }
}
