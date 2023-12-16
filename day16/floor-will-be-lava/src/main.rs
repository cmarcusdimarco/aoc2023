use std::{collections::HashSet, fs};

enum Direction {
    North,
    East,
    South,
    West,
}

fn parse_contraption(path: &str) -> Vec<String> {
    let input = fs::read_to_string(path).expect("Error reading file at specified path");
    let mut contraption: Vec<String> = Vec::new();

    for line in input.lines() {
        contraption.push(String::from(line));
    }

    contraption
}

fn energize_tiles(contraption: &Vec<String>) -> HashSet<(usize, usize)> {
    let mut energized_tiles: HashSet<(usize, usize)> = HashSet::new();

    // We can recurse through the path of the grid in order to more easily support
    // the multiple pathing.
    visit_tile((0, 0), Direction::West, contraption, &mut energized_tiles);

    energized_tiles
}

fn visit_tile(
    point: (usize, usize),
    from: Direction,
    contraption: &Vec<String>,
    visited: &mut HashSet<(usize, usize)>,
) {
    // If the point is within the grid, insert into the visited set
    visited.insert(point);

    // Check that the next point will not be beyond the indices of the grid
    if point.0 == 0
        || point.1 == 0
        || point.0 == contraption[0].len() - 1
        || point.1 == contraption.len() - 1
    {
        return;
    }

    // Move on to the next point
    match contraption[point.1].chars().nth(point.0).unwrap() {
        '\\' => match from {
            Direction::North => visit_tile(
                (point.0 + 1, point.1),
                Direction::West,
                contraption,
                visited,
            ),
            Direction::East => visit_tile(
                (point.0, point.1 - 1),
                Direction::South,
                contraption,
                visited,
            ),
            Direction::South => visit_tile(
                (point.0 - 1, point.1),
                Direction::East,
                contraption,
                visited,
            ),
            Direction::West => visit_tile(
                (point.0, point.1 + 1),
                Direction::North,
                contraption,
                visited,
            ),
        },
        '/' => match from {
            Direction::North => visit_tile(
                (point.0 - 1, point.1),
                Direction::East,
                contraption,
                visited,
            ),
            Direction::East => visit_tile(
                (point.0, point.1 + 1),
                Direction::North,
                contraption,
                visited,
            ),
            Direction::South => visit_tile(
                (point.0 + 1, point.1),
                Direction::West,
                contraption,
                visited,
            ),
            Direction::West => visit_tile(
                (point.0, point.1 - 1),
                Direction::South,
                contraption,
                visited,
            ),
        },
        '-' => match from {
            Direction::East => visit_tile((point.0 - 1, point.1), from, contraption, visited),
            Direction::West => visit_tile((point.0 + 1, point.1), from, contraption, visited),
            Direction::North | Direction::South => {
                visit_tile(
                    (point.0 - 1, point.1),
                    Direction::West,
                    contraption,
                    visited,
                );
                visit_tile(
                    (point.0 + 1, point.1),
                    Direction::East,
                    contraption,
                    visited,
                );
            }
        },
        '|' => match from {
            Direction::North => visit_tile((point.0, point.1 + 1), from, contraption, visited),
            Direction::South => visit_tile((point.0, point.1 - 1), from, contraption, visited),
            Direction::East | Direction::West => {
                visit_tile(
                    (point.0, point.1 + 1),
                    Direction::North,
                    contraption,
                    visited,
                );
                visit_tile(
                    (point.0, point.1 - 1),
                    Direction::South,
                    contraption,
                    visited,
                );
            }
        },
        _ => (),
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_contraption() {
        let expected = vec![
            ".|...\\....",
            "|.-.\\.....",
            ".....|-...",
            "........|.",
            "..........",
            ".........\\",
            "..../.\\\\..",
            ".-.-/..|..",
            ".|....-|.\\",
            "..//.|....",
        ];

        assert_eq!(expected, parse_contraption("test.txt"))
    }

    #[test]
    fn energizes_tiles() {
        let input = parse_contraption("test.txt");
        let actual: HashSet<(usize, usize)> = energize_tiles(&input);

        assert_eq!(46, actual.len())
    }
}
