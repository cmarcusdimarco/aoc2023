use std::fs;

#[derive(Debug, PartialEq, Eq)]
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

fn calculate_possible_arrangements(row: &Row) {
    // Our strategy will be to recurse through valid combinations and break out early
    // if there is a conflict between the combination and the Row's sequence.
    //
    // There are 2^n possible combinations, where n is the number of Condition::Unknowns.
    //
    // We can reduce this number firstly by calculating the difference between the sum
    // of a Row's sequence and the amount of Condition::Operationals present.
    let current_operationals = row.springs.iter().fold(0u32, |mut acc, x| { if let Condition::Operational = x { acc += 1;} acc } );
    let target_operationals: u32 = row.sequence.iter().sum();
    let quantity_to_change = target_operationals - current_operationals;

    
}

fn main() {
    println!("Hello, world!");
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
}
