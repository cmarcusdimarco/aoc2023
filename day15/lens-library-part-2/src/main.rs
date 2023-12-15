use std::{collections::HashMap, fs};

#[derive(Clone, Debug, PartialEq, Eq)]
struct Lens {
    label: String,
    focal_length: u32,
}

impl Lens {
    fn new(label: &str, focal_length: u32) -> Self {
        Self {
            label: String::from(label),
            focal_length,
        }
    }
}

fn parse_sequence(path: &str) -> Vec<String> {
    let input = fs::read_to_string(path).expect("Error reading file at specified path");
    let mut sequence: Vec<String> = Vec::new();

    for segment in input.split(',') {
        sequence.push(String::from(segment));
    }

    sequence
}

fn compute_hash(s: &str) -> u8 {
    let mut current_value: u32 = 0;

    for c in s.as_bytes().iter() {
        current_value += u32::from(*c);
        current_value *= 17;
        current_value %= 256;
    }

    u8::try_from(current_value).expect("Should be safe due to the previous mod 256 operation")
}

fn perform_operation(s: &str, boxes: &mut HashMap<u8, Vec<Lens>>) {
    if s.contains('=') {
        let (label, focal_length) = s.split_once('=').unwrap();
        let box_id = compute_hash(label);
        let new_lens = Lens::new(
            label,
            focal_length
                .parse::<u32>()
                .expect("Focal length should be a number"),
        );
        boxes
            .entry(box_id)
            .and_modify(|x| {
                match x
                    .iter()
                    .enumerate()
                    .find(|(_index, lens)| lens.label == label)
                {
                    Some((index, _lens)) => {
                        x[index] = new_lens.clone();
                    }
                    None => x.push(new_lens.clone()),
                }
            })
            .or_insert(vec![new_lens]);
    } else {
        let (label, _) = s.split_once('-').unwrap();
        let box_id = compute_hash(label);
        // If the box has been entered, check its contents for a lens with a matching label
        if let Some(_) = boxes.get(&box_id) {
            boxes.entry(box_id).and_modify(|x| {
                match x
                    .iter()
                    .enumerate()
                    .find(|(_index, lens)| lens.label == label)
                {
                    Some((index, _lens)) => {
                        x.remove(index);
                    }
                    None => (),
                }
            });
        }
    }
}

fn compute_total_focusing_power(boxes: &HashMap<u8, Vec<Lens>>) -> u32 {
    boxes.into_iter().fold(0u32, |acc, (id, lenses)| {
        acc + lenses
            .iter()
            .enumerate()
            .fold(0u32, |inner_acc, (index, lens)| {
                inner_acc
                    + u32::from(id + 1)
                        * u32::try_from(index + 1).expect("Couldn't convert index into u32")
                        * (lens.focal_length)
            })
    })
}

fn main() {
    let mut boxes: HashMap<u8, Vec<Lens>> = HashMap::new();
    let input = parse_sequence("input.txt");

    for lens in input.iter() {
        perform_operation(lens, &mut boxes);
    }

    println!(
        "The sum of the hashes is: {:?}",
        compute_total_focusing_power(&boxes)
    );
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
        assert_eq!(30u8, compute_hash("rn=1"))
    }

    #[test]
    fn performs_operation() {
        let mut expected: HashMap<u8, Vec<Lens>> = HashMap::new();
        let mut actual: HashMap<u8, Vec<Lens>> = HashMap::new();
        let input = parse_sequence("test.txt");

        expected.insert(0, vec![Lens::new("rn", 1), Lens::new("cm", 2)]);
        expected.insert(1, Vec::new());
        expected.insert(
            3,
            vec![Lens::new("ot", 7), Lens::new("ab", 5), Lens::new("pc", 6)],
        );

        for lens in input.iter() {
            perform_operation(lens, &mut actual);
        }

        assert_eq!(expected, actual)
    }

    #[test]
    fn computes_total_focusing_power() {
        let mut actual: HashMap<u8, Vec<Lens>> = HashMap::new();
        let input = parse_sequence("test.txt");

        for lens in input.iter() {
            perform_operation(lens, &mut actual);
        }

        assert_eq!(145, compute_total_focusing_power(&actual))
    }
}
