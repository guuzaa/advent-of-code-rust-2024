advent_of_code::solution!(12);

fn coords_of_letter(
    grid: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    x: isize,
    y: isize,
    letter: char,
) -> Vec<(isize, isize)> {
    if x < 0 || y < 0 || x >= grid[0].len() as isize || y >= grid.len() as isize {
        return vec![];
    }
    if visited[y as usize][x as usize] || grid[y as usize][x as usize] != letter {
        return vec![];
    }

    visited[y as usize][x as usize] = true;
    let mut coords = vec![(x, y)];

    // Check all 4 directions
    for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let mut additional = coords_of_letter(grid, visited, x + dx, y + dy, letter);
        coords.append(&mut additional);
    }
    coords
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let height = grid.len();
    let width = grid[0].len();
    let mut visited = vec![vec![false; width]; height];
    let mut total_price = 0;

    for y in 0..height {
        for x in 0..width {
            if visited[y][x] {
                continue;
            }
            let letter = grid[y][x];
            let region = coords_of_letter(&grid, &mut visited, x as isize, y as isize, letter);

            let area = region.len() as u32;

            let mut perimeter = 0;
            for &(x, y) in &region {
                for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let nx = x + dx;
                    let ny = y + dy;

                    if nx < 0
                        || ny < 0
                        || nx >= width as isize
                        || ny >= height as isize
                        || grid[ny as usize][nx as usize] != letter
                    {
                        perimeter += 1;
                    }
                }
            }

            total_price += area * perimeter;
        }
    }

    Some(total_price)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    #[ignore = "inputs"]
    fn test_part_one_inputs() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1467094));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
