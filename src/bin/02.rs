advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let count = input
        .lines()
        .filter(|line| !line.is_empty())
        .filter(|line| {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();

            // Check if sequence is valid
            if numbers.len() < 2 {
                return false;
            }

            // Determine if sequence should be increasing or decreasing
            let increasing = numbers[1] > numbers[0];

            // Check each adjacent pair
            numbers.windows(2).all(|pair| {
                let diff = pair[1] - pair[0];
                if increasing {
                    (1..=3).contains(&diff)
                } else {
                    (-3..0).contains(&diff)
                }
            })
        })
        .count() as u32;

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let count = input
        .lines()
        .filter(|line| !line.is_empty())
        .filter(|line| {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();

            // If it's already safe, no need to remove anything
            if is_safe(&numbers) {
                return true;
            }

            // Try removing each number one at a time
            for skip_idx in 0..numbers.len() {
                let modified: Vec<i32> = numbers
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i != skip_idx)
                    .map(|(_, &n)| n)
                    .collect();

                if is_safe(&modified) {
                    return true;
                }
            }
            false
        })
        .count() as u32;

    Some(count)
}

// Helper function to check if a sequence is safe
fn is_safe(numbers: &[i32]) -> bool {
    if numbers.len() < 2 {
        return false;
    }

    // Determine if sequence should be increasing or decreasing
    let increasing = numbers[1] > numbers[0];

    // Check each adjacent pair
    numbers.windows(2).all(|pair| {
        let diff = pair[1] - pair[0];
        if increasing {
            (1..=3).contains(&diff)
        } else {
            (-3..0).contains(&diff)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
