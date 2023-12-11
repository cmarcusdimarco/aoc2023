use std::fs;

#[derive(PartialEq, Eq)]
enum Direction {
    N,
    E,
    S,
    W,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Point {
    line: usize,
    column: usize,
}

impl Point {
    fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Pipe {
    shape: char,
    position: Point,
}

impl Pipe {
    fn new(shape: char, line: usize, column: usize) -> Self {
        Self {
            shape,
            position: Point { line, column },
        }
    }

    fn from(&self, from: &Direction) -> Option<Direction> {
        match self.shape {
            '|' if *from == Direction::N => Some(Direction::S),
            '|' if *from == Direction::S => Some(Direction::N),
            '-' if *from == Direction::E => Some(Direction::W),
            '-' if *from == Direction::W => Some(Direction::E),
            'L' if *from == Direction::N => Some(Direction::E),
            'L' if *from == Direction::E => Some(Direction::N),
            'J' if *from == Direction::N => Some(Direction::W),
            'J' if *from == Direction::W => Some(Direction::N),
            '7' if *from == Direction::W => Some(Direction::S),
            '7' if *from == Direction::S => Some(Direction::W),
            'F' if *from == Direction::E => Some(Direction::S),
            'F' if *from == Direction::S => Some(Direction::E),
            _ => None,
        }
    }
}

fn parse_path(filepath: &str) -> Vec<Pipe> {
    let input = fs::read_to_string(filepath)
        .expect("There was an issue reading the file at the specified path");

    let mut path: Vec<Pipe> = Vec::new();

    for (index, line) in input.lines().enumerate() {
        let position = line.find('S');

        if let Some(_) = position {
            path.push(Pipe::new('S', index, position.unwrap()));
            break;
        }
    }

    // Once the starting position is set, check the adjacent positions for
    // pipes, since our starting position character does not indicate flow.
    let lines: Vec<&str> = input.lines().collect();
    let starting_row = path[0].position.line;
    let starting_column = path[0].position.column;
    let mut traveled_from: Direction;

    'initial_direction: loop {
        // Check top
        if starting_row > 0 {
            match lines[starting_row - 1]
                .chars()
                .nth(starting_column)
                .expect("Position mismatch")
            {
                c if c == '|' || c == '7' || c == 'F' => {
                    path.push(Pipe::new(c, starting_row - 1, starting_column));
                    traveled_from = Direction::S;
                    break 'initial_direction;
                }
                _ => (),
            }
        }

        // Check right
        if starting_column < lines[starting_row].len() - 1 {
            match lines[starting_row]
                .chars()
                .nth(starting_column + 1)
                .expect("Position mismatch")
            {
                c if c == '-' || c == '7' || c == 'J' => {
                    path.push(Pipe::new(c, starting_row, starting_column + 1));
                    traveled_from = Direction::W;
                    break 'initial_direction;
                }
                _ => (),
            }
        }

        // Check bottom
        if starting_row < lines.len() - 1 {
            match lines[starting_row + 1]
                .chars()
                .nth(starting_column)
                .expect("Position mismatch")
            {
                c if c == '|' || c == 'L' || c == 'J' => {
                    path.push(Pipe::new(c, starting_row + 1, starting_column));
                    traveled_from = Direction::N;
                    break 'initial_direction;
                }
                _ => (),
            }
        }

        // Check left
        if starting_column > 0 {
            match lines[starting_row]
                .chars()
                .nth(starting_column - 1)
                .expect("Position mismatch")
            {
                c if c == '-' || c == 'L' || c == 'F' => {
                    path.push(Pipe::new(c, starting_row, starting_column - 1));
                    traveled_from = Direction::E;
                    break 'initial_direction;
                }
                _ => (),
            }
        }
    }

    let mut pipe_counter: usize = 1;

    'follow_path: loop {
        let current_pipe: &Pipe = &path[pipe_counter];

        // Find the next pipe
        let traveling_to = current_pipe
            .from(&traveled_from)
            .expect("There was an error in the pipe loop");

        let next_point = match traveling_to {
            Direction::N => {
                Point::new(current_pipe.position.line - 1, current_pipe.position.column)
            }
            Direction::E => {
                Point::new(current_pipe.position.line, current_pipe.position.column + 1)
            }
            Direction::S => {
                Point::new(current_pipe.position.line + 1, current_pipe.position.column)
            }
            Direction::W => {
                Point::new(current_pipe.position.line, current_pipe.position.column - 1)
            }
        };

        let next_pipe = Pipe {
            shape: lines[next_point.line]
                .chars()
                .nth(next_point.column)
                .expect("Point out of bounds of possible characters"),
            position: next_point,
        };

        // Break out of the loop if we have made it back to the start
        if next_pipe.shape == 'S' {
            break 'follow_path;
        }

        // Push to our path vector and reassign pointers
        path.push(next_pipe);
        pipe_counter += 1;
        traveled_from = match traveling_to {
            Direction::N => Direction::S,
            Direction::E => Direction::W,
            Direction::S => Direction::N,
            Direction::W => Direction::E,
        };
    }

    path
}

fn count_interior_points(grid: String, pipe_path: &mut Vec<Pipe>) -> u32 {
    // In order to not have to guess about the shape of the 'S' pipe,
    //   we'll replace it here with the corresponding shape.
    // To not alter the data in a way that's unexpected, we'll sacrifice
    //   some memory to clone `path`.
    let mut path = pipe_path.clone();

    let direction_a = match path[1].position {
        Point { line, .. } if line == path[0].position.line - 1 => Direction::N,
        Point { line: _, column } if column == path[0].position.column + 1 => Direction::E,
        Point { line, .. } if line == path[0].position.line + 1 => Direction::S,
        Point { line: _, column } if column == path[0].position.column - 1 => Direction::W,
        _ => panic!("Path was found to be disjoint"),
    };

    let direction_b = match path[path.len() - 1].position {
        Point { line, .. } if line == path[0].position.line - 1 => Direction::N,
        Point { line: _, column } if column == path[0].position.column + 1 => Direction::E,
        Point { line, .. } if line == path[0].position.line + 1 => Direction::S,
        Point { line: _, column } if column == path[0].position.column - 1 => Direction::W,
        _ => panic!("Path was found to be disjoint"),
    };

    // Since we started the discovery from the top of 'S', there are only six valid combinations:
    path[0].shape = match (direction_a, direction_b) {
        (Direction::N, Direction::E) => 'L',
        (Direction::N, Direction::S) => '|',
        (Direction::N, Direction::W) => 'J',
        (Direction::E, Direction::S) => 'F',
        (Direction::E, Direction::W) => '-',
        (Direction::S, Direction::W) => '7',
        _ => panic!("Encountered a combination of Directions that should be impossible"),
    };

    // Let's establish what we can that is concrete:
    //   1. If we scan from left to right for each line, the
    //      first pipe that we hit will be an outer wall of
    //      the shape.
    //   2. The only possible shapes for the first pipe in each
    //      line are '|', 'S', 'F', and 'L' (and we replaced 'S').
    //   3. We can set a flag to signal if we should count a given
    //      point based on the conclusions we can draw about the
    //      overall shape from the first pipe we encounter.
    let mut counter = 0;

    for (line_index, line) in grid.lines().enumerate() {
        let mut previous_corner = '.';
        let mut is_interior = false;
        let mut accumulator = 0;

        for (c_index, c) in line.chars().enumerate() {
            // If c is a pipe in the path...
            if path
                .iter()
                .any(|x| x.position.line == line_index && x.position.column == c_index)
            {
                // ...toggle the flag based on the shape of the pipe.
                match c {
                    '|' => is_interior = !is_interior,
                    'J' if previous_corner == 'F' => is_interior = !is_interior,
                    '7' if previous_corner == 'L' => is_interior = !is_interior,
                    'F' => {
                        previous_corner = 'F';
                    }
                    'L' => {
                        previous_corner = 'L';
                    }
                    _ => (),
                }

                if !is_interior {
                    counter += accumulator;
                    accumulator = 0;
                }
            } else {
                if is_interior {
                    accumulator += 1;
                }
            }
        }
    }

    counter
}

fn main() {
    let mut path = parse_path("input.txt");

    let interior_points =
        count_interior_points(fs::read_to_string("input.txt").unwrap(), &mut path);

    println!("The number of interior points is: {:?}", interior_points);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_path() {
        let expected = vec![
            Pipe::new('S', 1, 1),
            Pipe::new('-', 1, 2),
            Pipe::new('-', 1, 3),
            Pipe::new('-', 1, 4),
            Pipe::new('-', 1, 5),
            Pipe::new('-', 1, 6),
            Pipe::new('-', 1, 7),
            Pipe::new('7', 1, 8),
            Pipe::new('|', 2, 8),
            Pipe::new('|', 3, 8),
            Pipe::new('|', 4, 8),
            Pipe::new('|', 5, 8),
            Pipe::new('|', 6, 8),
            Pipe::new('J', 7, 8),
            Pipe::new('-', 7, 7),
            Pipe::new('-', 7, 6),
            Pipe::new('L', 7, 5),
            Pipe::new('|', 6, 5),
            Pipe::new('F', 5, 5),
            Pipe::new('-', 5, 6),
            Pipe::new('J', 5, 7),
            Pipe::new('|', 4, 7),
            Pipe::new('|', 3, 7),
            Pipe::new('7', 2, 7),
            Pipe::new('-', 2, 6),
            Pipe::new('-', 2, 5),
            Pipe::new('-', 2, 4),
            Pipe::new('-', 2, 3),
            Pipe::new('F', 2, 2),
            Pipe::new('|', 3, 2),
            Pipe::new('|', 4, 2),
            Pipe::new('L', 5, 2),
            Pipe::new('-', 5, 3),
            Pipe::new('7', 5, 4),
            Pipe::new('|', 6, 4),
            Pipe::new('J', 7, 4),
            Pipe::new('-', 7, 3),
            Pipe::new('-', 7, 2),
            Pipe::new('L', 7, 1),
            Pipe::new('|', 6, 1),
            Pipe::new('|', 5, 1),
            Pipe::new('|', 4, 1),
            Pipe::new('|', 3, 1),
            Pipe::new('|', 2, 1),
        ];

        assert_eq!(expected, parse_path("test1.txt"))
    }

    #[test]
    fn counts_interior_points() {
        let mut path = parse_path("test2.txt");

        assert_eq!(
            8,
            count_interior_points(fs::read_to_string("test2.txt").unwrap(), &mut path)
        )
    }
}
