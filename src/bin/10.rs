advent_of_code::solution!(10);

use std::collections::{HashSet, VecDeque};

type CountingFunction = fn(&[Vec<u32>], usize, usize) -> u32;

fn solve(input: &str, counting_fn: CountingFunction) -> Option<u32> {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();

    Some(
        (0..rows)
            .flat_map(|i| (0..cols).map(move |j| (i, j)))
            .filter(|&(i, j)| grid[i][j] == 0)
            .map(|(i, j)| counting_fn(&grid, i, j))
            .sum(),
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, count_reachable_nines)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, count_paths)
}

fn count_reachable_nines(grid: &[Vec<u32>], start_i: usize, start_j: usize) -> u32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut reachable_nines = HashSet::new();

    // Start BFS from the trailhead
    queue.push_back((start_i, start_j, 0)); // (row, col, current_height)
    visited.insert((start_i, start_j));

    while let Some((i, j, height)) = queue.pop_front() {
        // If we reached a 9, add it to our set of reachable nines
        if grid[i][j] == 9 {
            reachable_nines.insert((i, j));
            continue;
        }

        // Check all four directions
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // right, down, left, up
        for (di, dj) in directions {
            let ni = (i as i32 + di) as usize;
            let nj = (j as i32 + dj) as usize;
            if ni >= rows || nj >= cols {
                continue;
            }

            // Check if it's a valid next step (exactly one higher) and not visited
            if grid[ni][nj] == height + 1 && !visited.contains(&(ni, nj)) {
                queue.push_back((ni, nj, grid[ni][nj]));
                visited.insert((ni, nj));
            }
        }
    }

    reachable_nines.len() as u32
}

fn count_paths(grid: &[Vec<u32>], start_i: usize, start_j: usize) -> u32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = vec![vec![false; cols]; rows];

    fn dfs(
        grid: &[Vec<u32>],
        i: usize,
        j: usize,
        visited: &mut Vec<Vec<bool>>,
        rows: usize,
        cols: usize,
    ) -> u32 {
        // If we reached a 9, we found a valid path
        if grid[i][j] == 9 {
            return 1;
        }

        let current_height = grid[i][j];
        let mut total_paths = 0;
        visited[i][j] = true;

        // Check all four directions
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for (di, dj) in directions {
            let ni = (i as i32 + di) as usize;
            let nj = (j as i32 + dj) as usize;

            if ni >= rows || nj >= cols || visited[ni][nj] {
                continue;
            }

            // Only proceed if next height is exactly one more than current
            if grid[ni][nj] == current_height + 1 {
                total_paths += dfs(grid, ni, nj, visited, rows, cols);
            }
        }

        visited[i][j] = false; // Backtrack
        total_paths
    }

    dfs(grid, start_i, start_j, &mut visited, rows, cols)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    #[ignore = "inputs"]
    fn test_part_one_inputs() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(709));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }

    #[test]
    #[ignore = "inputs"]
    fn test_part_two_inputs() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1326));
    }
}
