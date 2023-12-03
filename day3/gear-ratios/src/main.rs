// Day 3, part 1 - Gear Ratios
use std::{collections::HashMap, error::Error, fs};

// A valid "part number" is any number in the input adjacent to a symbol -
// which can be defined as any non-whitespace character that is not a digit
// or a period.
#[derive(Clone)]
struct PartNumber {
    value: u32,
    length: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read the input
    let input = fs::read_to_string("input.txt")?;

    let mut previous_values: HashMap<usize, PartNumber> = HashMap::new();
    let mut previous_symbols: HashMap<usize, char> = HashMap::new();
    let mut valid_part_numbers: Vec<u32> = Vec::new();

    // For each line of input...
    for line in input.lines() {
        let mut value_aggregator = String::from("");
        let mut current_values: HashMap<usize, PartNumber> = HashMap::new();
        let mut current_symbols: HashMap<usize, char> = HashMap::new();
        let mut previous_char = '.';
        let mut current_symbol_adjacent = false;
        let mut previous_symbol_adjacent = false;

        // ...walk through the chars...
        for (index, char) in line.chars().enumerate() {
            match char {
                c if c.is_ascii_digit() => {
                    // ..check for inline matches...
                    if previous_char != '.' && !previous_char.is_ascii_digit() {
                        current_symbol_adjacent = true;
                    }

                    // ...and check for previous matches...
                    if previous_symbols.contains_key(&index)
                        || index > 0 && previous_symbols.contains_key(&(index - 1))
                    {
                        previous_symbol_adjacent = true;
                    }

                    value_aggregator.push_str(&c.to_string());
                }
                c if c != '.' => {
                    current_symbols.insert(index, c);
                    // If we encounter a symbol and the previous char is a digit, this is a valid inline part number
                    if previous_char.is_ascii_digit() {
                        valid_part_numbers.push(
                            value_aggregator
                                .parse::<u32>()
                                .expect("value_aggregator contained a non-digit value"),
                        );
                        reset_aggregator_and_flags(
                            &mut value_aggregator,
                            &mut current_symbol_adjacent,
                            &mut previous_symbol_adjacent,
                        );
                    }

                    // Check for previous values that this symbol is adjacent to
                    for (position, entry) in previous_values.clone().iter() {
                        // The range that indicates that a given value is a valid part number
                        // is the index preceding its start position through the index directly after
                        // the value's final digit.
                        let range = match position {
                            0 => *position..*position + entry.length + 1,
                            _ => *position - 1..*position + entry.length + 1,
                        };

                        if range.contains(&index) {
                            let entry = previous_values.remove(position).unwrap();
                            valid_part_numbers.push(entry.value);
                        }
                    }
                }
                _ if value_aggregator.len() > 0 => {
                    // Check for adjacent symbols in the preceding line
                    if previous_symbols.contains_key(&index) {
                        previous_symbol_adjacent = true;
                    }

                    // If either previous_symbol_adjacent or current_symbol_adjacent is true,
                    // the value in the aggregator is a valid part number.
                    if previous_symbol_adjacent || current_symbol_adjacent {
                        valid_part_numbers.push(
                            value_aggregator
                                .parse::<u32>()
                                .expect("value_aggregator contained a non-digit value"),
                        );
                    } else {
                        current_values.insert(
                            index - value_aggregator.len(),
                            PartNumber {
                                value: value_aggregator
                                    .parse::<u32>()
                                    .expect("value_aggregator contained a non-digit value"),
                                length: value_aggregator.len(),
                            },
                        );
                    }

                    reset_aggregator_and_flags(
                        &mut value_aggregator,
                        &mut current_symbol_adjacent,
                        &mut previous_symbol_adjacent,
                    );
                }
                _ => (),
            }

            previous_char = char;
        }

        // Handle final values, if present
        match value_aggregator {
            s if current_symbol_adjacent => valid_part_numbers.push(
                s.parse::<u32>()
                    .expect("value_aggregator contained a non-digit value"),
            ),
            s if s.len() > 0 => {
                // This statement needs to be terminated by a semicolon to tell the compiler
                // that we don't want to return its value to the match statement. This way,
                // the return value is the unit type () just like the branch above.
                current_values.insert(
                    line.len() - s.len(),
                    PartNumber {
                        value: s
                            .parse::<u32>()
                            .expect("value_aggregator contained a non-digit value"),
                        length: s.len(),
                    },
                );
            }
            _ => (),
        }

        // Move current symbols and values to their previous Vector counterparts
        // to be able to compare to new lines as we continue to iterate through the input.
        previous_symbols = current_symbols;
        previous_values = current_values;
    }

    let sum: u32 = valid_part_numbers.into_iter().sum();

    println!("The sum of the valid part numbers is: {:?}", sum);

    Ok(())
}

fn reset_aggregator_and_flags(agg: &mut String, current: &mut bool, previous: &mut bool) {
    *agg = String::from("");
    *current = false;
    *previous = false;
}
