use std::fs;

fn parse_sequence(path: &str) -> Vec<String> {
    let input = fs::read_to_string(path).expect("Error reading file at specified path");
    let mut sequence: Vec<String> = Vec::new();

    for segment in input.split(',') {
        sequence.push(String::from(segment));
    }

    sequence
}

fn compute_hash(s: &str) -> u32 {
    let mut current_value: u32 = 0;

    for c in s.as_bytes().iter() {
        current_value += u32::from(*c);
        current_value *= 17;
        current_value %= 256;
    }

    current_value
}

fn main() {
    let input = parse_sequence("input.txt");
    let sum = input.iter().fold(0u32, |acc, s| acc + compute_hash(s));

    println!("The sum of the hashes is: {:?}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_sequence() {
        let expected = vec![
            String::from("rn=1"),
            String::from("cm-"),
            String::from("qp=3"),
            String::from("cm=2"),
            String::from("qp-"),
            String::from("pc=4"),
            String::from("ot=9"),
            String::from("ab=5"),
            String::from("pc-"),
            String::from("pc=6"),
            String::from("ot=7"),
        ];

        assert_eq!(expected, parse_sequence("test.txt"))
    }

    #[test]
    fn computes_hash() {
        let input = parse_sequence("test.txt");
        let actual = input.iter().fold(0u32, |acc, s| acc + compute_hash(s));

        assert_eq!(1320, actual)
    }
}
