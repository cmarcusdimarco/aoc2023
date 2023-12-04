use std::{collections::HashMap, error::Error, fs, ops::Range};

fn main() -> Result<(), Box<dyn Error>> {
    // Read input
    let input = fs::read_to_string("input.txt")?;
    let mut map: HashMap<u32, u32> = HashMap::new();

    // For each line of the input...
    for line in input.lines() {
        // ...split the line into winning_numbers and numbers_present
        let numbers;
        let scratchcard_id: u32;
        let mut winning_numbers: Vec<u32> = Vec::new();
        let mut numbers_present: u32 = 0;

        match line.split_once(": ") {
            Some((a, b)) => {
                match a.split_once(' ') {
                    Some((_, b)) => scratchcard_id = b.trim().parse::<u32>().unwrap(),
                    None => panic!("There was an error parsing the id of the scratchcard"),
                }

                numbers = b;
            }
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
            }
            None => panic!("There was an error parsing the values in substring `numbers`"),
        }

        // Account for the original card
        let quantity_won = map
            .entry(scratchcard_id)
            .and_modify(|counter| *counter += 1)
            .or_insert(1u32)
            .clone();

        // Add the cards won to the map, where the quantity won is the quantity of the current card in the map.
        if numbers_present > 0 {
            let card_ids_won: Range<u32> = scratchcard_id + 1..scratchcard_id + numbers_present + 1;

            for id in card_ids_won.into_iter() {
                map.entry(id)
                    .and_modify(|counter| *counter += quantity_won)
                    .or_insert(quantity_won);
            }
        }
    }

    let total_cards: u32 = map.into_values().sum();

    println!("The total number of scratchcards is: {:?}", total_cards);

    Ok(())
}
