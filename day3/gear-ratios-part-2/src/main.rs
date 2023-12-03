// Day 3, part 2 - Gear Ratios
use std::{collections::HashMap, error::Error, fs};

// A valid "part number" is any number in the input adjacent to a symbol -
// which can be defined as any non-whitespace character that is not a digit
// or a period.
#[derive(Clone)]
struct PartNumber {
    value: u32,
    length: usize,
}

// We need Gears to implement the Clone trait so we can mutate their fields
// and then replace the original Gear in the HashMap. This is to comply with
// Rust's borrow-checker not allowing us to mutate from a borrow from HashMap::get().
#[derive(Clone)]
struct Gear {
    first_value: u32,
    second_value: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read the input
    let input = fs::read_to_string("input.txt")?;

    let mut previous_values: HashMap<usize, PartNumber> = HashMap::new();
    let mut previous_symbols: HashMap<usize, Gear> = HashMap::new();
    let mut valid_gear_ratios: Vec<u32> = Vec::new();

    // For each line of input...
    for line in input.lines() {
        let mut value_aggregator = String::from("");
        let mut current_values: HashMap<usize, PartNumber> = HashMap::new();
        let mut current_symbols: HashMap<usize, Gear> = HashMap::new();
        let mut previous_char = '.';
        let mut current_symbol_adjacent = (false, 0);
        let mut previous_symbol_adjacent = (false, 0);

        // ...walk through the chars...
        for (index, char) in line.chars().enumerate() {
            match char {
                c if c.is_ascii_digit() => {
                    // ..check for inline matches...
                    if previous_char == '*' {
                        current_symbol_adjacent = (true, index - 1);
                    }

                    // ...and check for previous matches...
                    if previous_symbols.contains_key(&index) {
                        previous_symbol_adjacent = (true, index);
                    }

                    if index > 0 && previous_symbols.contains_key(&(index - 1)) {
                        previous_symbol_adjacent = (true, index - 1);
                    }

                    value_aggregator.push_str(&c.to_string());
                }
                c if c == '*' => {
                    current_symbols.insert(
                        index,
                        Gear {
                            first_value: 0,
                            second_value: 0,
                        },
                    );

                    // If we encounter a symbol and the previous char is a digit, push the value to the gear.
                    if previous_char.is_ascii_digit() {
                        let mut gear: Gear = current_symbols.get(&index).unwrap().clone();
                        let value = value_aggregator
                            .parse::<u32>()
                            .expect("value_aggregator contained a non-digit value");

                        add_value_to_gear(&mut gear, value);

                        current_symbols.insert(index, gear);

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
                            let mut gear = current_symbols.get(&index).unwrap().clone();
                            let value = entry.value;

                            add_value_to_gear(&mut gear, value);

                            current_symbols.insert(index, gear);
                        }
                    }
                }
                _ if value_aggregator.len() > 0 => {
                    // Check for adjacent symbols in the preceding line
                    if previous_symbols.contains_key(&index) {
                        previous_symbol_adjacent = (true, index);
                    }

                    let value = value_aggregator
                        .parse::<u32>()
                        .expect("value_aggregator contained a non-digit value");

                    // If either previous_symbol_adjacent or current_symbol_adjacent is true,
                    // the value in the aggregator is a valid part number.
                    if previous_symbol_adjacent.0 {
                        let mut gear = previous_symbols
                            .get(&previous_symbol_adjacent.1)
                            .expect("Error looking up gear from previous_symbols")
                            .clone();

                        add_value_to_gear(&mut gear, value);

                        previous_symbols.insert(previous_symbol_adjacent.1, gear);
                    } else if current_symbol_adjacent.0 {
                        let mut gear = current_symbols
                            .get(&current_symbol_adjacent.1)
                            .expect("Error looking up gear from current_symbols")
                            .clone();

                        add_value_to_gear(&mut gear, value);

                        current_symbols.insert(current_symbol_adjacent.1, gear);
                    } else {
                        current_values.insert(
                            index - value_aggregator.len(),
                            PartNumber {
                                value,
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
            s if current_symbol_adjacent.0 => {
                let mut gear = current_symbols
                    .get(&current_symbol_adjacent.1)
                    .unwrap()
                    .clone();
                let value = s
                    .parse::<u32>()
                    .expect("value_aggregator contained a non-digit value");

                add_value_to_gear(&mut gear, value);

                current_symbols.insert(current_symbol_adjacent.1, gear);
            }
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

        // Iterate over previous_symbols to find Gears that have two values
        for gear in previous_symbols.into_values() {
            if gear.first_value > 0 && gear.second_value > 0 {
                valid_gear_ratios.push(gear.first_value * gear.second_value);
            }
        }

        // Move current symbols and values to their previous Vector counterparts
        // to be able to compare to new lines as we continue to iterate through the input.
        previous_symbols = current_symbols;
        previous_values = current_values;
    }

    let sum: u32 = valid_gear_ratios.into_iter().sum();

    println!("The sum of the valid part numbers is: {:?}", sum);

    Ok(())
}

fn reset_aggregator_and_flags(
    agg: &mut String,
    current: &mut (bool, usize),
    previous: &mut (bool, usize),
) {
    *agg = String::from("");
    *current = (false, 0);
    *previous = (false, 0);
}

fn add_value_to_gear(gear: &mut Gear, value: u32) {
    match gear {
        g if g.first_value == 0 => g.first_value = value,
        g => g.second_value = value,
    }
}
