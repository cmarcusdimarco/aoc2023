use std::fs;

#[derive(PartialEq, Eq)]
enum Direction {
    N,
    E,
    S,
    W,
}

#[derive(Debug, PartialEq, Eq)]
struct Point {
    line: usize,
    column: usize,
}

impl Point {
    fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

#[derive(Debug, PartialEq, Eq)]
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

fn calculate_steps_to_farthest_point(path: &Vec<Pipe>) -> usize {
    path.len() / 2
}

fn main() {
    let path = parse_path("input.txt");

    println!(
        "The amount of steps needed to reach the farthest point of the path is: {:?}",
        calculate_steps_to_farthest_point(&path)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_path() {
        let expected = vec![
            Pipe::new('S', 2, 0),
            Pipe::new('J', 2, 1),
            Pipe::new('F', 1, 1),
            Pipe::new('J', 1, 2),
            Pipe::new('F', 0, 2),
            Pipe::new('7', 0, 3),
            Pipe::new('|', 1, 3),
            Pipe::new('L', 2, 3),
            Pipe::new('7', 2, 4),
            Pipe::new('J', 3, 4),
            Pipe::new('-', 3, 3),
            Pipe::new('-', 3, 2),
            Pipe::new('F', 3, 1),
            Pipe::new('J', 4, 1),
            Pipe::new('L', 4, 0),
            Pipe::new('|', 3, 0),
        ];

        assert_eq!(expected, parse_path("test.txt"))
    }

    #[test]
    fn calculates_steps_to_farthest_point() {
        let path = parse_path("test.txt");

        assert_eq!(8usize, calculate_steps_to_farthest_point(&path))
    }
}
