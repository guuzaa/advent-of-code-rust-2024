advent_of_code::solution!(8);

use std::collections::{HashMap, HashSet};

pub fn part_one(input: &str) -> Option<u32> {
    let mut antennas: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    let grid: Vec<&str> = input.lines().collect();
    let height = grid.len() as isize;
    let width = grid.iter().map(|line| line.len()).max().unwrap_or(0) as isize;

    for (y, line) in grid.iter().enumerate() {
        for (x, &c) in line.as_bytes().iter().enumerate() {
            if c.is_ascii_alphanumeric() {
                antennas
                    .entry(c as char)
                    .or_default()
                    .push((x as isize, y as isize));
            }
        }
    }

    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();

    // Calculate antinodes for each pair of antennas with the same frequency
    for (_freq, positions) in antennas.iter() {
        for i in 0..positions.len() {
            for j in (i + 1)..positions.len() {
                let (x1, y1) = positions[i];
                let (x2, y2) = positions[j];

                let dx = x2 - x1;
                let dy = y2 - y1;

                // Calculate the two possible antinode positions
                let antinode1 = (x1 - dx, y1 - dy);
                let antinode2 = (x2 + dx, y2 + dy);

                // Check if antinodes are within the grid bounds
                if antinode1.0 >= 0
                    && antinode1.0 < width
                    && antinode1.1 >= 0
                    && antinode1.1 < height
                {
                    antinodes.insert(antinode1);
                }
                if antinode2.0 >= 0
                    && antinode2.0 < width
                    && antinode2.1 >= 0
                    && antinode2.1 < height
                {
                    antinodes.insert(antinode2);
                }
            }
        }
    }

    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    // Parse the grid and collect antenna positions by frequency
    let mut antennas: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    let grid: Vec<&str> = input.lines().collect();
    let height = grid.len() as isize;
    let width = grid.iter().map(|line| line.len()).max().unwrap_or(0) as isize;

    for (y, line) in grid.iter().enumerate() {
        for (x, &c) in line.as_bytes().iter().enumerate() {
            if c.is_ascii_alphanumeric() {
                antennas
                    .entry(c as char)
                    .or_default()
                    .push((x as isize, y as isize));
            }
        }
    }

    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();

    for (_freq, positions) in antennas.iter() {
        for i in 0..positions.len() {
            for j in (i + 1)..positions.len() {
                let (x1, y1) = positions[i];
                let (x2, y2) = positions[j];
                antinodes.insert((x1, y1));
                antinodes.insert((x2, y2));

                let dx = x2 - x1;
                let dy = y2 - y1;

                let mut antinode1 = (x1 - dx, y1 - dy);
                while antinode1.0 >= 0
                    && antinode1.0 < width
                    && antinode1.1 >= 0
                    && antinode1.1 < height
                {
                    antinodes.insert(antinode1);
                    antinode1.0 -= dx;
                    antinode1.1 -= dy;
                }

                let mut antinode2 = (x2 + dx, y2 + dy);
                while antinode2.0 >= 0
                    && antinode2.0 < width
                    && antinode2.1 >= 0
                    && antinode2.1 < height
                {
                    antinodes.insert(antinode2);
                    antinode2.0 += dx;
                    antinode2.1 += dy;
                }
            }
        }
    }

    Some(antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    #[ignore = "inputs"]
    fn test_part_one_inputs() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(276));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }

    #[test]
    #[ignore = "inputs"]
    fn test_part_two_inputs() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(991));
    }
}
