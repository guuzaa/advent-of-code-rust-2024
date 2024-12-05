advent_of_code::solution!(5);

use std::collections::{HashMap, HashSet};

pub fn part_one(input: &str) -> Option<u32> {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let mut rules: HashMap<u32, HashSet<u32>> = HashMap::new();
    for line in parts[0].lines() {
        if line.is_empty() {
            continue;
        }
        let nums: Vec<u32> = line.split('|').filter_map(|s| s.parse().ok()).collect();
        rules.entry(nums[0]).or_default().insert(nums[1]);
    }

    let mut sum = 0;
    for update in parts[1].lines() {
        if update.is_empty() {
            continue;
        }

        // Parse update numbers
        let pages: Vec<u32> = update.split(',').filter_map(|s| s.parse().ok()).collect();

        if is_valid_order(&pages, &rules) {
            let mid_idx = pages.len() / 2;
            sum += pages[mid_idx];
        }
    }

    Some(sum)
}

fn is_valid_order(pages: &[u32], rules: &HashMap<u32, HashSet<u32>>) -> bool {
    pages.iter().enumerate().all(|(i, &page)| {
        rules.get(&page).map_or(true, |must_come_after| {
            // For each page that must come after the current page
            pages[..i]
                .iter()
                .all(|&must_follow| !must_come_after.contains(&must_follow))
        })
    })
}

pub fn part_two(input: &str) -> Option<u32> {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let mut rules: HashMap<u32, HashSet<u32>> = HashMap::new();
    for line in parts[0].lines() {
        if line.is_empty() {
            continue;
        }
        let nums: Vec<u32> = line.split('|').filter_map(|s| s.parse().ok()).collect();
        rules.entry(nums[0]).or_default().insert(nums[1]);
    }

    let mut sum = 0;
    for update in parts[1].lines() {
        if update.is_empty() {
            continue;
        }

        // Parse update numbers
        let mut pages: Vec<u32> = update.split(',').filter_map(|s| s.parse().ok()).collect();
        // Only process updates that are not in valid order
        if !is_valid_order(&pages, &rules) {
            // Sort the pages according to rules
            sort_pages(&mut pages, &rules);

            // Add middle number of correctly sorted update
            let mid_idx = pages.len() / 2;
            sum += pages[mid_idx];
        }
    }

    Some(sum)
}

fn sort_pages(pages: &mut [u32], rules: &HashMap<u32, HashSet<u32>>) {
    let len = pages.len();
    let mut swapped;

    for i in 0..len {
        swapped = false;
        for j in 0..len - i - 1 {
            let a = pages[j];
            let b = pages[j + 1];

            // Check if b must come before a
            let mut should_swap = false;
            if let Some(after_b) = rules.get(&b) {
                if after_b.contains(&a) {
                    should_swap = true;
                }
            }

            if should_swap {
                pages.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    #[ignore = "inputs"]
    fn test_part_one_input() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(5964));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }

    #[test]
    #[ignore = "inputs"]
    fn test_part_two_input() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(4719));
    }
}
