use std::fs;

fn parse_histories(path: &str) -> Vec<Vec<i32>> {
    let input = fs::read_to_string(path).expect("There was an error reading the file");
    let mut histories: Vec<Vec<i32>> = Vec::new();

    for line in input.lines() {
        let mut history: Vec<i32> = Vec::new();

        for number in line.split_whitespace() {
            history.push(number.parse().expect("The input only contain numbers"));
        }

        histories.push(history);
    }

    histories
}

fn predict_next_value(history: Vec<i32>) -> i32 {
    let mut patterns: Vec<Vec<i32>> = vec![history];
    let mut current_index: usize = 0;

    // Iterate over sliding windows to calculate the difference between them
    const WINDOW_SIZE: usize = 2;

    'outer: loop {
        let mut non_zero_detected = false;
        let mut current_pattern: Vec<i32> = Vec::new();

        for pair in patterns[current_index].windows(WINDOW_SIZE) {
            let value = pair[1] - pair[0];

            if value != 0 {
                non_zero_detected = true;
            }

            current_pattern.push(value);
        }

        patterns.push(current_pattern);
        current_index += 1;

        if !non_zero_detected {
            break 'outer;
        }
    }

    // With the pattern vectors built out, the sum of the final elements of two
    // adjacent vectors is the predicted next value of the lower-indexed vector.

    // Knowing this, we can simply fold the final elements together and return the sum.
    let sum = patterns.iter().fold(0, |acc, x| acc + x.last().unwrap());

    sum
}

fn main() {
    let histories = parse_histories("input.txt");

    let sum = histories
        .into_iter()
        .fold(0, |acc, x| acc + predict_next_value(x));

    println!("The sum of all predicted next values is: {:?}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_histories() {
        let expected = vec![
            vec![0, 3, 6, 9, 12, 15],
            vec![1, 3, 6, 10, 15, 21],
            vec![10, 13, 16, 21, 30, 45],
        ];

        assert_eq!(expected, parse_histories("test.txt"))
    }

    #[test]
    fn predicts_next_value() {
        let expected = vec![18, 28, 68];
        let actual = vec![
            predict_next_value(vec![0, 3, 6, 9, 12, 15]),
            predict_next_value(vec![1, 3, 6, 10, 15, 21]),
            predict_next_value(vec![10, 13, 16, 21, 30, 45]),
        ];

        assert_eq!(expected, actual)
    }
}
