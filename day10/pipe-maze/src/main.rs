use std::fs;

fn parse_path(filepath: &str) {
    let input = fs::read_to_string(filepath)
        .expect("There was an issue reading the file at the specified path");

    let mut path_points: Vec<(usize, usize)> = Vec::new();
    let mut path = String::from("");

    for (index, line) in input.lines().enumerate() {
        let position = line.find('S');

        if let Some(_) = position {
            path_points.push((index, position.unwrap()));
            path.push_str("S");
            break;
        }
    }

    // Once the starting position is set, check the adjacent positions for
    // pipes, since our starting position character does not indicate flow.
    let lines: Vec<&str> = input.lines().collect();
    let starting_row = path_points[0].0;
    let starting_column = path_points[0].1;
    let mut next_point;

    // Check top
    if path.len() == 1 && starting_row > 0 {
        match lines[starting_row - 1].char_at(starting_column) {
            '|' => 
        }
    }
}

fn main() {
    println!("Hello, world!");
}
