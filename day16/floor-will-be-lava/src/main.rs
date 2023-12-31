use std::{
    collections::{HashMap, VecDeque},
    fs,
};

#[derive(Clone, Debug, PartialEq, Eq)]
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

fn energize_tiles(
    contraption: &Vec<String>,
    beam_a_start: (i8, i8, Direction),
) -> HashMap<(usize, usize), Vec<Direction>> {
    let mut energized_tiles: HashMap<(usize, usize), Vec<Direction>> = HashMap::new();
    let mut queue: VecDeque<(i8, i8, Direction)> = VecDeque::new();

    // We can recurse through the path of the grid in order to more easily support
    // the multiple pathing.
    queue.push_back(beam_a_start);

    while queue.len() > 0 {
        let (col, row, direction) = queue.pop_front().unwrap();
        visit_tile(
            (col, row),
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
    visited: &mut HashMap<(usize, usize), Vec<Direction>>,
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

    println!("Visiting point {:?}", point);

    // If the point is within the grid,
    // check to see if we have been here before from the same direction.
    // If so, exit.
    // If not, insert.
    let entry = visited.get_mut(&visited_point);

    match entry {
        Some(list) if list.contains(&from) => return,
        Some(list) => list.push(from.clone()),
        None => {
            visited.insert(visited_point, vec![from.clone()]);
        }
    }

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
                queue.push_back((point.0 - 1, point.1, Direction::East));
                queue.push_back((point.0 + 1, point.1, Direction::West));
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
    let mut max: usize = 0;
    let directions = vec![
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    for direction in directions.iter() {
        for i in 0..contraption.len() {
            let i = i8::try_from(i).unwrap();
            let length = i8::try_from(contraption.len() - 1).unwrap();

            let point: (i8, i8, Direction) = match direction {
                Direction::North => (i, 0, direction.clone()),
                Direction::East => (length, i, direction.clone()),
                Direction::South => (i, length, direction.clone()),
                Direction::West => (0, i, direction.clone()),
            };

            let energized_tiles = energize_tiles(&contraption, point);

            max = std::cmp::max(max, energized_tiles.len());
        }
    }

    println!("The maximum amount of tiles energized is: {:?}", max);
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
        let contraption = parse_contraption("test.txt");
        let mut max: usize = 0;
        let directions = vec![
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ];

        for direction in directions.iter() {
            for i in 0..contraption.len() {
                let i = i8::try_from(i).unwrap();
                let length = i8::try_from(contraption.len() - 1).unwrap();

                let point: (i8, i8, Direction) = match direction {
                    Direction::North => (i, 0, direction.clone()),
                    Direction::East => (length, i, direction.clone()),
                    Direction::South => (i, length, direction.clone()),
                    Direction::West => (0, i, direction.clone()),
                };

                let energized_tiles = energize_tiles(&contraption, point);

                max = std::cmp::max(max, energized_tiles.len());
            }
        }

        assert_eq!(51, max)
    }
}
