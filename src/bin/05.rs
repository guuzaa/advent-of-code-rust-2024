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
    for (i, &page) in pages.iter().enumerate() {
        // Check if this page has any rules
        if let Some(must_come_after) = rules.get(&page) {
            // For each page that must come after the current page
            for &must_follow in must_come_after {
                // If the page that must come after is in this update but appears before
                if pages[..i].contains(&must_follow) {
                    return false;
                }
            }
        }

        // Check reverse rules - if this page must come after any other pages
        for (&before_page, after_pages) in rules {
            if after_pages.contains(&page) {
                // If we find this page in a later position, the before_page must appear earlier
                if !pages[..i].contains(&before_page) && pages.contains(&before_page) {
                    return false;
                }
            }
        }
    }
    true
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
    for _ in 0..len {
        for j in 0..len - 1 {
            let a = pages[j];
            let b = pages[j + 1];

            // Check if b must come before a
            let mut should_swap = false;

            // Check direct rule b -> a
            if let Some(after_b) = rules.get(&b) {
                if after_b.contains(&a) {
                    should_swap = true;
                }
            }

            // Check if a appears in any "must come after" sets where b is not yet seen
            for (&before_page, after_pages) in rules {
                if after_pages.contains(&a) && before_page == b {
                    should_swap = true;
                }
            }

            if should_swap {
                pages.swap(j, j + 1);
            }
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
