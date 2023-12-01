use std::{fs, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;

    let calibration_values = &mut input.split_whitespace();

    let mut sum: u32 = 0;

    // For each line of input...
    for line in calibration_values {
        let mut first_digit: u32 = 0;
        let mut last_digit: u32 = 0;
        const RADIX: u32 = 10;

        // ...get the digit that appears first...
        for char in line.chars() {
            if char.is_ascii_digit() {
                first_digit = char.to_digit(RADIX).expect("Argument mismatch between char::is_ascii_digit and char::to_digit");
                break;
            }
        }

        // ...and the digit that appears last...
        for char in line.chars().rev() {
            if char.is_ascii_digit() {
                last_digit = char.to_digit(RADIX).expect("Argument mismatch between char::is_ascii_digit and char::to_digit");
                break;
            }
        }

        // ...and add them to the sum, where the first digit is the tens and the second is the ones.
        sum += (first_digit * 10) + last_digit;
    }

    println!("The sum of the calibration values is: {:?}", sum);

    Ok(())
}