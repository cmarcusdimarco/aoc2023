use core::panic;
use std::{fs, error::Error};
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;

    let calibration_values = &mut input.split_whitespace();

    let mut sum: u32 = 0;

    // Regex patterns
    let regex = Regex::new(r"[0-9]|zero|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let reversed_regex = Regex::new(r"[0-9]|orez|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();

    // For each line of input...
    for line in calibration_values {
        // ...get the digit that appears first...
        let first_match = regex.find(line).expect("There was an error in the forwards regex search.");
        let first_digit = match first_match.as_str() {
            "0" | "zero" => 0,
            "1" | "one" => 1,
            "2" | "two" => 2,
            "3" | "three" => 3,
            "4" | "four" => 4,
            "5" | "five" => 5,
            "6" | "six" => 6,
            "7" | "seven" => 7,
            "8" | "eight" => 8,
            "9" | "nine" => 9,
            _ => panic!("There was a critical error in parsing the forwards Regex match."),
        };

        // ...and the digit that appears last...
        let reversed_line = reverse_string(line);
        let last_match = reversed_regex.find(&reversed_line).expect("There was an error in the reverse regex search.");
        let last_digit = match reverse_string(last_match.as_str()).as_str() {
            "0" | "zero" => 0,
            "1" | "one" => 1,
            "2" | "two" => 2,
            "3" | "three" => 3,
            "4" | "four" => 4,
            "5" | "five" => 5,
            "6" | "six" => 6,
            "7" | "seven" => 7,
            "8" | "eight" => 8,
            "9" | "nine" => 9,
            _ => panic!("There was a critical error in parsing the reverse Regex match."),
        };

        // ...and add them to the sum, where the first digit is the tens and the second is the ones.
        sum += (first_digit * 10) + last_digit;
    }

    println!("The sum of the calibration values is: {:?}", sum);

    Ok(())
}

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}