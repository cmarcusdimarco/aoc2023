use std::fs;

#[derive(Debug, PartialEq, Eq)]
struct Point {
    line: usize,
    column: usize
}

impl Point {
    fn new(line: usize, column: usize) -> Self {
        Self {
            line, column
        }
    }
}

fn parse_image(path: &str) -> Vec<String> {
    let mut image: Vec<String> = Vec::new();

    let input = fs::read_to_string(path).expect("Unrecoverable error reading the file at the specified path");

    for line in input.lines() {
        let line = line.to_string();

        image.push(line);
    }

    image
}

// Galaxy expansion occurs where all elements in a row or column
// are empty space (.).
fn add_galaxy_expansion(image: &mut Vec<String>) {
    // We iterate in reverse so we can modify without worrying about altering
    // the upcoming indices
    for (index, row) in image.clone().iter().enumerate().rev() {
        if !row.contains('#') {
            // We will use the unused character '-' to represent the million empty rows.
            image[index] = row.replace('.', "-");
        }
    }

    for i in (0..image[0].len()).rev() {
        // For columns, we can iterate over all rows and check the nth character
        // in each row. If we encounter a galaxy (#), continue to the next index.
        if image.iter().any(|s| s.chars().nth(i).expect("All elements in the image should be of equal length") == '#') {
            continue;
        }

        // If we made it through the above guard clause, replace the column's characters.
        // We will use '!' to represent the million empty columns, and 'X' to represent the
        // intersection of '!' and '-'
        for s in image.iter_mut() {
            match s.chars().nth(i) {
                Some('.') => s.replace_range(i..i + 1, "!"),
                Some('-') => s.replace_range(i..i + 1, "X"),
                _ => (),
            }
        }
    }
}

fn locate_galaxies(image: &Vec<String>) -> Vec<Point> {
    let mut galaxies: Vec<Point> = Vec::new();

    // Account for the additional distance indicated by our new expansion characters
    // '-', '!', and 'X'
    let mut millions_of_rows: usize = 0;

    for (line_index, row) in image.iter().enumerate() {
        let mut millions_of_columns: usize = 0;
        let mut increment_mil_rows = false;

        for (column_index, ch) in row.chars().enumerate() {
            match ch {
                '!' => millions_of_columns += 1,
                '-' => increment_mil_rows = true,
                'X' => {
                    millions_of_columns += 1;
                    increment_mil_rows = true;
                },
                '#' => galaxies.push(Point::new(line_index + millions_of_rows * 1_000_000, column_index + millions_of_columns * 1_000_000)),
                _ => (),
            }
        }

        if increment_mil_rows {
            millions_of_rows += 1;
        }
    }

    galaxies
}

fn calculate_sum_of_paths(galaxies: &Vec<Point>) -> usize {
    let mut sum = 0;

    for i in 0..galaxies.len() - 1 {
        for j in i + 1..galaxies.len() {
            let Point { line: line_i, column: column_i } = galaxies[i];
            let Point { line: line_j, column: column_j } = galaxies[j];

            let horizontal_travel = std::cmp::max(column_i, column_j) - std::cmp::min(column_i, column_j);
            let vertical_travel = std::cmp::max(line_i, line_j) - std::cmp::min(line_i, line_j);

            sum += horizontal_travel + vertical_travel;
        }
    }

    sum
}

fn main() {
    let mut image = parse_image("input.txt");
    add_galaxy_expansion(&mut image);
    let galaxies = locate_galaxies(&image);

    println!("The sum of the paths between all pairs of galaxies is: {:?}", calculate_sum_of_paths(&galaxies));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_image() {
        let expected = vec![
            "...#......",
            ".......#..",
            "#.........",
            "..........",
            "......#...",
            ".#........",
            ".........#",
            "..........",
            ".......#..",
            "#...#.....",
        ];

        assert_eq!(expected, parse_image("test.txt"));
    }

    #[test]
    fn adds_galaxy_expansion() {
        let expected = vec![
            "....#........",
            ".........#...",
            "#............",
            ".............",
            ".............",
            "........#....",
            ".#...........",
            "............#",
            ".............",
            ".............",
            ".........#...",
            "#....#.......",
        ];

        let mut actual = parse_image("test.txt");
        add_galaxy_expansion(&mut actual);

        assert_eq!(expected, actual)
    }

    #[test]
    fn locates_galaxies() {
        let expected = vec![
            Point::new(0, 4),
            Point::new(1, 9),
            Point::new(2, 0),
            Point::new(5, 8),
            Point::new(6, 1),
            Point::new(7, 12),
            Point::new(10, 9),
            Point::new(11, 0),
            Point::new(11, 5),
        ];

        let mut actual = parse_image("test.txt");
        add_galaxy_expansion(&mut actual);

        assert_eq!(expected, locate_galaxies(&actual))
    }

    #[test]
    fn calculates_sum_of_paths() {
        let mut actual = parse_image("test.txt");
        add_galaxy_expansion(&mut actual);
        let galaxies = locate_galaxies(&actual);

        assert_eq!(374usize, calculate_sum_of_paths(&galaxies))
    }
}