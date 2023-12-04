use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    // Read input
    let input = fs::read_to_string("input.txt")?;
    let mut points: u32 = 0;

    // For each line of the input...
    for line in input.lines() {
        // ...split the line into winning_numbers and numbers_present
        let numbers;
        let mut winning_numbers: Vec<u32> = Vec::new();
        let mut numbers_present: u32 = 0;

        match line.split_once(": ") {
            Some((_, b)) => numbers = b,
            None => panic!("There was an error splitting the input line."),
        }

        match numbers.split_once(" | ") {
            Some((a, b)) => {
                for value in a.split_whitespace() {
                    winning_numbers.push(value.parse::<u32>().unwrap());
                }

                // Sort winning_numbers to improve search time
                winning_numbers.sort();

                for value in b.split_whitespace() {
                    let value = value.parse::<u32>().unwrap();
                
                    if let Result::Ok(_) = winning_numbers.binary_search(&value) {
                        numbers_present += 1;
                    }
                }
            },
            None => panic!("There was an error parsing the values in substring `numbers`"),
        }

        // Increase points by the result of 2 raised to the power of winning values present
        if numbers_present > 0 {
            points += 2u32.pow(numbers_present - 1);
        }
    }

    println!("The total number of winning points is: {:?}", points);

    Ok(())
}
