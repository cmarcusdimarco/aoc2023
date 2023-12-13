use std::{collections::HashMap, fs};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Row {
    springs: Vec<Condition>,
    sequence: Vec<usize>,
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
                .parse::<usize>()
                .expect("Non-numeric value present in damaged_spring_sequence");

            row.sequence.push(segment);
        }

        // In part 2, we learn that for each row of input, we must push four extra copies of the row to get the
        // real input.
        let condition_clone = row.springs.clone();
        let sequence_clone = row.sequence.clone();

        for _ in 0..4 {
            row.springs.push(Condition::Unknown);
            row.springs.append(&mut condition_clone.clone());
            row.sequence.append(&mut sequence_clone.clone());
        }

        rows.push(row);
    }

    rows
}

fn calculate_possible_arrangements(row: &Row, cache: &mut HashMap<Row, u64>) -> u64 {
    // Our strategy will be to recurse through valid combinations and break out early
    // if there is a conflict between the combination and the Row's sequence.

    // Start the recursion
    recurse(row.springs.clone(), row.sequence.clone(), cache)
}

// Recursion simplification and caching inspiration drawn from HyperNeutrino's Python solution on YouTube
// Our original implementation...was not very performant
fn recurse(springs: Vec<Condition>, sequence: Vec<usize>, cache: &mut HashMap<Row, u64>) -> u64 {
    // Memoize results
    let row = Row {
        springs: springs.clone(),
        sequence: sequence.clone(),
    };

    if let Some(_) = cache.get(&row) {
        return *cache.get(&row).unwrap();
    }

    // Base cases
    if springs.len() == 0 {
        return match sequence {
            s if s.len() == 0 => 1,
            _ => 0,
        };
    } else if sequence.len() == 0 {
        return match springs {
            s if s.contains(&Condition::Damaged) => 0,
            _ => 1,
        };
    }

    let mut result = 0;

    if springs[0] != Condition::Damaged {
        result += recurse(springs[1..].to_vec(), sequence.clone(), cache);
    }

    if springs[0] != Condition::Operational {
        if sequence[0] <= springs.len()
            && !springs[..sequence[0]]
                .iter()
                .any(|x| *x == Condition::Operational)
            && (sequence[0] == springs.len() || springs[sequence[0]] != Condition::Damaged)
        {
            let sliced_springs = match sequence[0] {
                x if x == springs.len() => Vec::new(),
                x => springs[x + 1..].to_vec(),
            };

            result += recurse(sliced_springs, sequence[1..].to_vec(), cache)
        }
    }

    cache.insert(row, result);

    result
}

fn main() {
    let mut cache: HashMap<Row, u64> = HashMap::new();
    let rows = parse_rows("input.txt");
    let sum = rows.iter().fold(0u64, |acc, x| {
        acc + calculate_possible_arrangements(x, &mut cache)
    });

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
        let mut cache: HashMap<Row, u64> = HashMap::new();

        let actual = rows.iter().fold(0u64, |acc, x| {
            acc + calculate_possible_arrangements(x, &mut cache)
        });

        assert_eq!(525152, actual)
    }
}
