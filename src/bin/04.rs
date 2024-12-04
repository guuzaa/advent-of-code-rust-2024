advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;
    let directions = [
        (0, 1),   // right
        (1, 0),   // down
        (1, 1),   // diagonal down-right
        (-1, 1),  // diagonal up-right
        (0, -1),  // left
        (-1, 0),  // up
        (-1, -1), // diagonal up-left
        (1, -1),  // diagonal down-left
    ];
    let target = "XMAS";

    // Check all possible starting positions
    for row in 0..rows {
        for col in 0..cols {
            // Define all 8 directions to check

            // Check each direction from current position
            for (dx, dy) in directions {
                if target.chars().enumerate().all(|(i, expected_char)| {
                    let new_row = row as i32 + dx * i as i32;
                    let new_col = col as i32 + dy * i as i32;
                    new_row >= 0
                        && new_row < rows as i32
                        && new_col >= 0
                        && new_col < cols as i32
                        && grid[new_row as usize][new_col as usize] == expected_char
                }) {
                    count += 1;
                }
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    let forward = "MAS";
    let backward = "SAM";
    // Check all four possible combinations:
    // 1. Both forward MAS
    // 2. Both backward SAM
    // 3. Forward MAS + backward SAM
    // 4. Backward SAM + forward MAS
    let combinations = [
        (forward, forward),
        (backward, backward),
        (forward, backward),
        (backward, forward),
    ];

    // Check each possible center point of the X
    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            for (first_pattern, second_pattern) in combinations.iter() {
                if first_pattern.chars().enumerate().all(|(i, expected_char)| {
                    let new_row = (row as i32 + i as i32 - 1) as usize;
                    let new_col = (col as i32 + i as i32 - 1) as usize;
                    grid[new_row][new_col] == expected_char
                }) && second_pattern
                    .chars()
                    .enumerate()
                    .all(|(i, expected_char)| {
                        let new_row = (row as i32 + i as i32 - 1) as usize;
                        let new_col = (col as i32 - i as i32 + 1) as usize;
                        grid[new_row][new_col] == expected_char
                    })
                {
                    count += 1;
                }
            }
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
