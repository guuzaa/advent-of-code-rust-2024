use std::collections::HashSet;
advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = map.len();
    let cols = map[0].len();

    // Find starting position
    let mut pos = map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, &c)| (i, j, c)))
        .find(|&(_, _, c)| c == '^')
        .map(|(i, j, _)| (i, j))
        .unwrap_or((0, 0));

    let mut visited = HashSet::new();
    visited.insert(pos);

    let mut direction = (-1, 0); // up
    loop {
        let next_row = (pos.0 as isize + direction.0) as usize;
        let next_col = (pos.1 as isize + direction.1) as usize;

        // Check if we're about to leave the map
        if next_row >= rows || next_col >= cols {
            break;
        }

        // Check if there's an obstacle ahead
        if map[next_row][next_col] == '#' {
            // Turn right 90 degrees
            direction = (direction.1, -direction.0);
        } else {
            // Move forward
            pos = (next_row, next_col);
            visited.insert(pos);
        }
    }

    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = map.len();
    let cols = map[0].len();

    // Find starting position
    let start_pos = map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, &c)| (i, j, c)))
        .find(|&(_, _, c)| c == '^')
        .map(|(i, j, _)| (i, j))
        .unwrap_or((0, 0));

    // Define direction offsets: North, East, South, West
    let offsets = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let max_steps = rows * cols + 1;
    let mut loop_positions = 0;

    // Try each position as a potential obstacle
    for i in 0..rows {
        for j in 0..cols {
            if map[i][j] != '.' {
                continue;
            }

            let mut pos = start_pos;
            let mut steps = 0;
            let mut direction = 0;

            loop {
                steps += 1;
                if steps > max_steps {
                    // We're in a loop!
                    loop_positions += 1;
                    break;
                }

                let next_row = pos.0 as isize + offsets[direction].0;
                let next_col = pos.1 as isize + offsets[direction].1;

                // Check if we're leaving the map
                if next_row < 0
                    || next_row >= rows as isize
                    || next_col < 0
                    || next_col >= cols as isize
                {
                    break;
                }

                let next_pos = (next_row as usize, next_col as usize);

                // Check if we hit our test obstacle
                if next_pos == (i, j) {
                    direction = (direction + 1) % 4;
                    continue;
                }

                // Check if path is clear
                if map[next_pos.0][next_pos.1] == '#' {
                    direction = (direction + 1) % 4;
                } else {
                    pos = next_pos; // Move forward
                }
            }
        }
    }

    Some(loop_positions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    #[ignore = "inputs"]
    fn test_part_one_inputs() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(5331));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    #[ignore = "inputs"]
    fn test_part_two_inputs() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1812));
    }
}
