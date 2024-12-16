use num_traits::Euclid;
use std::collections::HashMap;

advent_of_code::solution!(11);

fn simulate_blinking(stones: Vec<u64>, num_blinks: u32) -> u64 {
    let mut cache: HashMap<u64, u64> = HashMap::default();
    for stone in stones {
        cache.entry(stone).and_modify(|v| *v += 1).or_insert(1);
    }

    for _ in 0..num_blinks {
        let mut new_cache: HashMap<u64, u64> = HashMap::default();
        for (num, count) in cache.into_iter() {
            match num {
                0 => {
                    new_cache
                        .entry(1)
                        .and_modify(|v| *v += count)
                        .or_insert(count);
                }
                n if num_digits(n) % 2 == 0 => {
                    let num_digits = num_digits(n);
                    let (left, right) = n.div_rem_euclid(&10_u64.pow(num_digits / 2));
                    new_cache
                        .entry(left)
                        .and_modify(|v| *v += count)
                        .or_insert(count);
                    new_cache
                        .entry(right)
                        .and_modify(|v| *v += count)
                        .or_insert(count);
                }
                n => {
                    new_cache
                        .entry(n * 2024)
                        .and_modify(|v| *v += count)
                        .or_insert(count);
                }
            }
        }
        cache = new_cache;
    }

    cache.values().sum::<u64>()
}

fn num_digits(n: u64) -> u32 {
    n.checked_ilog10().unwrap_or(0) + 1
}

pub fn part_one(input: &str) -> Option<u64> {
    let stones = input
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    Some(simulate_blinking(stones, 25))
}

pub fn part_two(input: &str) -> Option<u64> {
    let stones = input
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    Some(simulate_blinking(stones, 75))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    #[ignore = "inputs"]
    fn test_part_one_inputs() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(207683));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }

    #[test]
    #[ignore = "inputs"]
    fn test_part_two_inputs() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(244782991106220));
    }
}
