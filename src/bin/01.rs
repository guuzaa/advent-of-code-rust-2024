use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    // Split input into lines and parse each line into two numbers
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let mut numbers = line.split_whitespace().map(|n| n.parse::<u32>().unwrap());

        left_list.push(numbers.next().unwrap());
        right_list.push(numbers.next().unwrap());
    }

    // Sort both lists
    left_list.sort();
    right_list.sort();

    // Calculate total distance
    let total_distance: u32 = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum();

    Some(total_distance)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left_list = Vec::new();
    let mut right_list_counter = HashMap::new();

    // Parse input into two lists
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let mut numbers = line.split_whitespace().map(|n| n.parse::<u32>().unwrap());
        left_list.push(numbers.next().unwrap());
        right_list_counter
            .entry(numbers.next().unwrap())
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }

    // Calculate similarity score
    let similarity_score: u32 = left_list
        .iter()
        .map(|&left_num| {
            let count = right_list_counter.get(&left_num).unwrap_or(&0);
            left_num * count
        })
        .sum();

    Some(similarity_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
