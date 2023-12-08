use std::{collections::HashMap, fs};

#[derive(Debug, PartialEq)]
struct Node {
    name: String,
    left: String,
    right: String
}

impl Node {
    fn empty() -> Self {
        Self {
            name: String::from(""),
            left: String::from(""),
            right: String::from("")
        }
    }

    fn new(name: &str, left: &str, right: &str) -> Self {
        Self {
            name: String::from(name),
            left: String::from(left),
            right: String::from(right)
        }
    }
}

fn parse_map(path: &str) -> (String, HashMap<String, Node>) {
    let input = fs::read_to_string(path).unwrap();

    // The first line of the input is the directions, and the contents thereafter
    // are the network.
    let destination;
    let mut network: HashMap<String, Node> = HashMap::new();

    match input.split_once('\n') {
        Some((a, b)) => {
            destination = String::from(a.trim());

            for line in b.trim().lines() {
                // Since we know the input format, we can process it based on the format.
                let line = line.replace("= (", "");
                let line = line.replace(",", "");
                let line = line.replace(")", "");

                let mut node = Node::empty();

                for (i, val) in line.split_whitespace().enumerate() {
                    match i {
                        0 => node.name = String::from(val),
                        1 => node.left = String::from(val),
                        2 => node.right = String::from(val),
                        _ => panic!("Index greater than 2 detected while parsing input."),
                    }
                }

                network.insert(node.name.clone(), node);
            }
        },
        _ => panic!("File contents did not match expected format. Please check contents and try again."),
    }

    (destination, network)
}

fn calculate_nodes_visited(destination: String, network: HashMap<String, Node>) -> u32 {
    let mut visited_counter:u32 = 0;
    let mut current_key = "AAA";

    'outer: while current_key != "ZZZ" {
        for step in destination.chars() {
            if current_key == "ZZZ" {
                break 'outer;
            }

            let current_node = network.get(current_key).expect("Key/value pair mismatch detected.");
            current_key = match step {
                'L' => &current_node.left,
                'R' => &current_node.right,
                _ => panic!("Directions contained a value other than L or R"),
            };
            visited_counter += 1;
        }
    }

    visited_counter
}

fn main() {
    let (destination, network) = parse_map("input.txt");

    println!("The number of steps required to reach ZZZ is: {:?}", calculate_nodes_visited(destination, network));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_map_1() {
        let mut expected_map: HashMap<String, Node> = HashMap::new();
        let network = vec![
            Node::new("AAA", "BBB", "CCC"),
            Node::new("BBB", "DDD", "EEE"),
            Node::new("CCC", "ZZZ", "GGG"),
            Node::new("DDD", "DDD", "DDD"),
            Node::new("EEE", "EEE", "EEE"),
            Node::new("GGG", "GGG", "GGG"),
            Node::new("ZZZ", "ZZZ", "ZZZ"),
        ];

        for node in network.into_iter() {
            expected_map.insert(node.name.clone(), node);
        }

        let expected = (String::from("RL"), expected_map);

        assert_eq!(expected, parse_map("test1.txt"))
    }

    #[test]
    fn parses_map_2() {
        let mut expected_map: HashMap<String, Node> = HashMap::new();
        let network = vec![
            Node::new("AAA", "BBB", "BBB"),
            Node::new("BBB", "AAA", "ZZZ"),
            Node::new("ZZZ", "ZZZ", "ZZZ"),
        ];

        for node in network.into_iter() {
            expected_map.insert(node.name.clone(), node);
        }

        let expected = (String::from("LLR"), expected_map);

        assert_eq!(expected, parse_map("test2.txt"))
    }

    #[test]
    fn calculates_nodes_visited_1() {
        let (destination, network) = parse_map("test1.txt");
        assert_eq!(2, calculate_nodes_visited(destination, network))
    }

    #[test]
    fn calculates_nodes_visited_2() {
        let (destination, network) = parse_map("test2.txt");
        assert_eq!(6, calculate_nodes_visited(destination, network))
    }
}