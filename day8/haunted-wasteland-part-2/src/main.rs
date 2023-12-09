use num_integer::Integer;
use std::{collections::HashMap, fs};

#[derive(Debug, PartialEq, Eq)]
struct Node {
    name: String,
    left: String,
    right: String,
}

impl Node {
    fn empty() -> Self {
        Self {
            name: String::from(""),
            left: String::from(""),
            right: String::from(""),
        }
    }

    fn new(name: &str, left: &str, right: &str) -> Self {
        Self {
            name: String::from(name),
            left: String::from(left),
            right: String::from(right),
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
        }
        _ => panic!(
            "File contents did not match expected format. Please check contents and try again."
        ),
    }

    (destination, network)
}

fn derive_starting_nodes(network: &HashMap<String, Node>) -> Vec<String> {
    let mut starting_nodes: Vec<String> = Vec::new();

    // Iterate over network keys to find all nodes ending with 'A'
    for key in network.keys() {
        if key.ends_with('A') {
            starting_nodes.push(key.clone());
        }
    }

    starting_nodes
}

fn calculate_nodes_visited(
    destination: String,
    network: HashMap<String, Node>,
    starting_nodes: Vec<String>,
) -> u64 {
    let mut visited_counter: u64 = 0;
    let target_completed_paths = starting_nodes.len();
    let mut current_keys = starting_nodes;
    let mut complete_paths: Vec<u64> = Vec::new();

    'outer: loop {
        for step in destination.chars() {
            let mut non_z_key_detected = false;
            let mut new_keys: Vec<String> = Vec::new();

            for key in current_keys.iter() {
                if !key.ends_with('Z') {
                    non_z_key_detected = true;
                } else {
                    complete_paths.push(visited_counter);

                    if complete_paths.len() == target_completed_paths {
                        break 'outer;
                    } else {
                        continue;
                    }
                }

                let current_node = network.get(key).expect("Key/value pair mismatch detected.");
                let new_key = match step {
                    'L' => current_node.left.clone(),
                    'R' => current_node.right.clone(),
                    _ => panic!("Directions contained a value other than L or R"),
                };

                new_keys.push(new_key);
            }

            // If all keys end in Z, break the loop
            if !non_z_key_detected {
                break 'outer;
            }

            current_keys = new_keys;
            visited_counter += 1;
        }
    }

    complete_paths
        .into_iter()
        .reduce(|acc, x| acc.lcm(&x))
        .unwrap()
}

fn main() {
    let (destination, network) = parse_map("input.txt");
    let starting_nodes = derive_starting_nodes(&network);

    println!(
        "The number of steps required to reach ZZZ is: {:?}",
        calculate_nodes_visited(destination, network, starting_nodes)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_map() {
        let mut expected_map: HashMap<String, Node> = HashMap::new();
        let network = vec![
            Node::new("11A", "11B", "XXX"),
            Node::new("11B", "XXX", "11Z"),
            Node::new("11Z", "11B", "XXX"),
            Node::new("22A", "22B", "XXX"),
            Node::new("22B", "22C", "22C"),
            Node::new("22C", "22Z", "22Z"),
            Node::new("22Z", "22B", "22B"),
            Node::new("XXX", "XXX", "XXX"),
        ];

        for node in network.into_iter() {
            expected_map.insert(node.name.clone(), node);
        }

        let expected = (String::from("LR"), expected_map);

        assert_eq!(expected, parse_map("test.txt"))
    }

    #[test]
    fn derives_starting_nodes() {
        let expected = vec!["11A", "22A"];

        let (_, network) = parse_map("test.txt");

        assert_eq!(expected, derive_starting_nodes(&network))
    }

    #[test]
    fn calculates_nodes_visited() {
        let (destination, network) = parse_map("test.txt");
        assert_eq!(
            6,
            calculate_nodes_visited(
                destination,
                network,
                vec![String::from("11A"), String::from("22A")]
            )
        )
    }
}
