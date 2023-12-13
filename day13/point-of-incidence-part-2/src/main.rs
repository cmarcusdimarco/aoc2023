use std::{collections::HashMap, fs};

#[derive(Debug, PartialEq, Eq)]
enum Axis {
    Row,
    Column
}

fn parse_patterns(path: &str) -> Vec<Vec<String>> {
    let input = fs::read_to_string(path).unwrap();
    let mut patterns: Vec<Vec<String>> = Vec::new();
    let mut current_pattern: Vec<String> = Vec::new();

    for line in input.lines() {
        match line {
            s if s.len() > 0 => current_pattern.push(String::from(s)),
            _ => {
                patterns.push(current_pattern.clone());
                current_pattern = Vec::new();
            }
        }
    }

    if current_pattern.len() > 0 {
        patterns.push(current_pattern);
    }

    patterns
}

fn find_reflection(pattern: &Vec<String>) -> Option<(Axis, usize)> {
    // Rows
    'outer: for (index, rows) in pattern.windows(2).enumerate() {
        if rows[0] != rows[1] {
            continue;
        }

        // If two adjacent rows are identical, move outward from the window
        // until a non-identical pair is found or we hit one end of the pattern.
        let mut i = index;
        let mut j = index + 1;

        while i > 0 && j < pattern.len() - 1 {
            i -= 1;
            j += 1;

            if pattern[i] != pattern[j] {
                continue 'outer;
            }
        }

        // If we made it this far, it's a match - return the lower index and the Axis.
        return Some((Axis::Row, index))
    }

    // Columns
    'outer: for index in 0..pattern[0].len() - 1 {
        if pattern.iter().any(|s| s.chars().nth(index).expect("Should not be an index violation") != s.chars().nth(index + 1).expect("Should not be an index violation")) {
            continue;
        }

        // If two adjacent columns are identical, move outward from the window
        // until a non-identical pair is found or we hit one end of the pattern.
        let mut i = index;
        let mut j = index + 1;

        while i > 0 && j < pattern[0].len() - 1 {
            i -= 1;
            j += 1;

            if pattern.iter().any(|s| s.chars().nth(i).unwrap() != s.chars().nth(j).unwrap()) {
                continue 'outer;
            }
        }

        // If we made it this far, it's a match - return the lower index and the Axis.
        return Some((Axis::Column, index))
    }

    None
}

fn find_alternate_reflection(pattern: &Vec<String>, original_reflection: Option<(Axis, usize)>) -> Option<(Axis, usize)> {
    // Rows
    'outer_row: for (index, rows) in pattern.windows(2).enumerate() {
        // Part 2 allows us to encounter 1 "mistake" and still complete as true.
        let mut mistake_found = false;
        if rows[0] != rows[1] {
            // Now if the rows are unequal, check if they differ by a single character.
            for (inner_index, c) in rows[0].char_indices() {
                if c != rows[1].chars().nth(inner_index).unwrap() {
                    if !mistake_found {
                        mistake_found = true;
                    } else {
                        continue 'outer_row;
                    }
                }
            }
        }

        // If two adjacent rows are identical, move outward from the window
        // until a non-identical pair is found or we hit one end of the pattern.
        let mut i = index;
        let mut j = index + 1;

        while i > 0 && j < pattern.len() - 1 {
            i -= 1;
            j += 1;

            if pattern[i] != pattern[j] {
                if mistake_found {
                    continue 'outer_row;
                }
                
                // Now if the rows are unequal, check if they differ by a single character.
                for (inner_index, c) in pattern[i].char_indices() {
                    if c != pattern[j].chars().nth(inner_index).unwrap() {
                        if !mistake_found {
                            mistake_found = true;
                        } else {
                            continue 'outer_row;
                        }
                    }
                }
            }
        }

        // If this was the original result, skip it.
        if original_reflection == Some((Axis::Row, index)) {
            continue;
        }

        // If we made it this far, it's a match - return the lower index and the Axis.
        return Some((Axis::Row, index))
    }

    // Columns
    'outer_column: for index in 0..pattern[0].len() - 1 {
        // Part 2 allows us to encounter 1 "mistake" and still complete as true.
        let mut mistake_found = false;
        for row in pattern.iter() {
            if row.chars().nth(index).unwrap() != row.chars().nth(index + 1).unwrap() {
                if !mistake_found {
                    mistake_found = true;
                } else {
                    continue 'outer_column;
                }
            }
        }

        // If two adjacent columns are identical, move outward from the window
        // until a non-identical pair is found or we hit one end of the pattern.
        let mut i = index;
        let mut j = index + 1;

        while i > 0 && j < pattern[0].len() - 1 {
            i -= 1;
            j += 1;

            for row in pattern.iter() {
                if row.chars().nth(i).unwrap() != row.chars().nth(j).unwrap() {
                    if !mistake_found {
                        mistake_found = true;
                    } else {
                        continue 'outer_column;
                    }
                }
            }
        }

        // If this was the original result, skip it.
        if original_reflection == Some((Axis::Column, index)) {
            continue;
        }

        // If we made it this far, it's a match - return the lower index and the Axis.
        return Some((Axis::Column, index))
    }

    None
}

fn main() {
    let patterns = parse_patterns("input.txt");
    let mut reflections: Vec<Option<(Axis, usize)>> = Vec::new();

    for pattern in patterns.iter() {
        let reflection = find_reflection(&pattern);
        let different_reflection = find_alternate_reflection(pattern, reflection);
        reflections.push(different_reflection);
    }

    let sum = reflections.iter().fold(0, |acc, x| if let Some(_) = x {
        match x.as_ref().unwrap() {
            (Axis::Row, i) => acc + (i + 1) * 100,
            (Axis::Column, i) => acc + i + 1,
        }
    } else {
        panic!("Value of None present in reflections");
    });

    println!("The sum of the reflections is: {:?}", sum);   
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_patterns() {
        let expected = vec![
            vec![
                "#.##..##.",
                "..#.##.#.",
                "##......#",
                "##......#",
                "..#.##.#.",
                "..##..##.",
                "#.#.##.#.",
            ],
            vec![           
                "#...##..#",
                "#....#..#",
                "..##..###",
                "#####.##.",
                "#####.##.",
                "..##..###",
                "#....#..#",
            ],
        ];

        assert_eq!(expected, parse_patterns("test.txt"))
    }

    #[test]
    fn finds_reflections() {
        let patterns = parse_patterns("test.txt");

        let expected: Vec<Option<(Axis, usize)>> = vec![
            Some((Axis::Row, 2)),
            Some((Axis::Row, 0)),
        ];
        let actual = vec![
            find_alternate_reflection(&patterns[0], find_reflection(&patterns[0])),
            find_alternate_reflection(&patterns[1], find_reflection(&patterns[1]))
        ];

        assert_eq!(expected, actual)
    }

    #[test]
    fn finds_sum() {
        let patterns = parse_patterns("test.txt");
        let reflections = vec![
            find_alternate_reflection(&patterns[0], find_reflection(&patterns[0])),
            find_alternate_reflection(&patterns[1], find_reflection(&patterns[1]))
        ];

        let sum = reflections.iter().fold(0, |acc, x| if let Some(_) = x {
            match x.as_ref().unwrap() {
                (Axis::Row, i) => acc + (i + 1) * 100,
                (Axis::Column, i) => acc + i + 1,
            }
        } else {
            panic!("Value of None present in reflections");
        });

        assert_eq!(400, sum)
    }
}