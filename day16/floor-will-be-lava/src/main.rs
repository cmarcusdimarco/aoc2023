use std::{
    collections::{HashSet, VecDeque},
    fs,
};

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
    let mut queue: VecDeque<(i8, i8, Direction)> = VecDeque::new();

    // We can recurse through the path of the grid in order to more easily support
    // the multiple pathing.
    queue.push_back((0, 0, Direction::West));

    while queue.len() > 0 {
        let (row, col, direction) = queue.pop_front().unwrap();
        visit_tile(
            (row, col),
            direction,
            contraption,
            &mut energized_tiles,
            &mut queue,
        );
    }

    energized_tiles
}

fn visit_tile(
    point: (i8, i8),
    from: Direction,
    contraption: &Vec<String>,
    visited: &mut HashSet<(usize, usize)>,
    queue: &mut VecDeque<(i8, i8, Direction)>,
) {
    // Check that the point will not be beyond the indices of the grid
    if point.0 < 0
        || point.1 < 0
        || point.0 >= contraption[0].len().try_into().unwrap()
        || point.1 >= contraption.len().try_into().unwrap()
    {
        return;
    }

    let visited_point = (
        usize::try_from(point.0).unwrap(),
        usize::try_from(point.1).unwrap(),
    );

    // If the point is within the grid, insert into the visited set
    visited.insert(visited_point);

    // Move on to the next point
    match contraption[visited_point.1]
        .chars()
        .nth(visited_point.0)
        .unwrap()
    {
        '\\' => match from {
            Direction::North => queue.push_back((point.0 + 1, point.1, Direction::West)),
            Direction::East => queue.push_back((point.0, point.1 - 1, Direction::South)),
            Direction::South => queue.push_back((point.0 - 1, point.1, Direction::East)),
            Direction::West => queue.push_back((point.0, point.1 + 1, Direction::North)),
        },
        '/' => match from {
            Direction::North => queue.push_back((point.0 - 1, point.1, Direction::East)),
            Direction::East => queue.push_back((point.0, point.1 + 1, Direction::North)),
            Direction::South => queue.push_back((point.0 + 1, point.1, Direction::West)),
            Direction::West => queue.push_back((point.0, point.1 - 1, Direction::South)),
        },
        '-' => match from {
            Direction::East => queue.push_back((point.0 - 1, point.1, from)),
            Direction::West => queue.push_back((point.0 + 1, point.1, from)),
            Direction::North | Direction::South => {
                queue.push_back((point.0 - 1, point.1, Direction::West));
                queue.push_back((point.0 + 1, point.1, Direction::East));
            }
        },
        '|' => match from {
            Direction::North => queue.push_back((point.0, point.1 + 1, from)),
            Direction::South => queue.push_back((point.0, point.1 - 1, from)),
            Direction::East | Direction::West => {
                queue.push_back((point.0, point.1 + 1, Direction::North));
                queue.push_back((point.0, point.1 - 1, Direction::South));
            }
        },
        '.' => match from {
            Direction::North => queue.push_back((point.0, point.1 + 1, from)),
            Direction::East => queue.push_back((point.0 - 1, point.1, from)),
            Direction::South => queue.push_back((point.0, point.1 - 1, from)),
            Direction::West => queue.push_back((point.0 + 1, point.1, from)),
        },
        _ => (),
    }
}

fn main() {
    let contraption = parse_contraption("input.txt");
    let energized_tiles = energize_tiles(&contraption);

    println!(
        "The amount of tiles energized is: {:?}",
        energized_tiles.len()
    );
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
