use std::fs;

fn parse_platform(path: &str) -> Vec<Vec<char>> {
    let input = fs::read_to_string(path).expect("Error reading file at specified path");
    let mut platform: Vec<Vec<char>> = Vec::new();
    
    for (index, line) in input.lines().enumerate() {
        platform.push(Vec::new());

        for c in line.chars() {
            platform[index].push(c);
        }
    }

    platform
}

fn roll_north(platform: &mut Vec<Vec<char>>) {
    // We'll take an approach of iterating through the columns,
    // starting from the final row. Sum the Os (round rocks) encountered
    // until hitting a # (cube rock), then replace the chars from rows
    // n + 1 through x - 1, where x is the index of the last # seen (or the
    // max value, if a # hasn't been seen yet.)
    for column in 0..platform[0].len() {
        
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_platform() {
        let expected = vec![
            vec!['O','.','.','.','.','#','.','.','.','.'],
            vec!['O','.','O','O','#','.','.','.','.','#'],
            vec!['.','.','.','.','.','#','#','.','.','.'],
            vec!['O','O','.','#','O','.','.','.','.','O'],
            vec!['.','O','.','.','.','.','.','O','#','.'],
            vec!['O','.','#','.','.','O','.','#','.','#'],
            vec!['.','.','O','.','.','#','O','.','.','O'],
            vec!['.','.','.','.','.','.','.','O','.','.'],
            vec!['#','.','.','.','.','#','#','#','.','.'],
            vec!['#','O','O','.','.','#','.','.','.','.'],
        ];

        assert_eq!(expected, parse_platform("test.txt"))
    }
}