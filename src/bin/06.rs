use std::collections::HashSet;
advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = map.len();
    let cols = map[0].len();

    // Find starting position
    let mut pos = (0, 0);
    for (i, row) in map.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == '^' {
                pos = (i, j);
                break;
            }
        }
    }

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
    let mut map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = map.len();
    let cols = map[0].len();

    // Find starting position
    let mut start_pos = (0, 0);
    for (i, row) in map.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == '^' {
                start_pos = (i, j);
                break;
            }
        }
    }
    map[start_pos.0][start_pos.1] = '.';

    // Track visited positions
    let mut visited = HashSet::new();
    let mut pos = start_pos;
    let mut direction = (-1, 0); // up

    // Simulate original path
    loop {
        let next_row = (pos.0 as isize + direction.0) as usize;
        let next_col = (pos.1 as isize + direction.1) as usize;

        if next_row >= rows || next_col >= cols {
            break;
        }

        if map[next_row][next_col] == '#' {
            direction = (direction.1, -direction.0);
        } else {
            pos = (next_row, next_col);
            visited.insert(pos);
        }
    }

    // Only test positions that were visited in original path
    let mut loop_positions = 0;
    for &pos in &visited {
        if pos != start_pos {
            map[pos.0][pos.1] = '#';
            if creates_loop(&map, start_pos) {
                loop_positions += 1;
            }
            map[pos.0][pos.1] = '.';
        }
    }

    Some(loop_positions)
}

fn creates_loop(map: &[Vec<char>], start: (usize, usize)) -> bool {
    let rows = map.len();
    let cols = map[0].len();
    let mut pos = start;
    let mut direction = (-1, 0);

    // Track position + direction combinations we've seen
    let mut visited = HashSet::new();
    visited.insert((pos, direction));

    loop {
        let next_row = (pos.0 as isize + direction.0) as usize;
        let next_col = (pos.1 as isize + direction.1) as usize;

        // Check if we're about to leave the map
        if next_row >= rows || next_col >= cols {
            return false;
        }

        // Check if there's an obstacle ahead
        if map[next_row][next_col] == '#' {
            // Turn right 90 degrees
            direction = (direction.1, -direction.0);
        } else {
            // Move forward
            pos = (next_row, next_col);
        }

        // Check if we've seen this position + direction before (indicates a loop)
        if !visited.insert((pos, direction)) {
            return true;
        }

        // Prevent infinite loops by limiting iterations
        if visited.len() > rows * cols * 4 {
            return false;
        }
    }
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
