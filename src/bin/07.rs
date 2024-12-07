advent_of_code::solution!(7);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u64> {
    let re = Regex::new(r"\d+").ok()?;
    let mut sum = 0;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let tokens: Vec<u64> = re
            .find_iter(line)
            .filter_map(|m| m.as_str().parse().ok())
            .collect();

        let result = tokens[0];
        let lhs = tokens[1];
        if add_mul(result, lhs, 2, &tokens) {
            sum += result;
        }
    }
    Some(sum)
}

fn add_mul(result: u64, lhs: u64, depth: usize, tokens: &[u64]) -> bool {
    if depth >= tokens.len() {
        return false;
    }

    let rhs = tokens[depth];
    let product = lhs * rhs;
    if product == result && depth == tokens.len() - 1 {
        return true;
    }
    if add_mul(result, product, depth + 1, tokens) {
        return true;
    }

    let sum = lhs + rhs;
    if sum == result && depth == tokens.len() - 1 {
        return true;
    }
    add_mul(result, sum, depth + 1, tokens)
}

pub fn part_two(input: &str) -> Option<u64> {
    let re = Regex::new(r"\d+").ok()?;
    let mut sum = 0;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let tokens: Vec<u64> = re
            .find_iter(line)
            .filter_map(|m| m.as_str().parse().ok())
            .collect();

        let result = tokens[0];
        let lhs = tokens[1];
        if add_mul_concat(result, lhs, 2, &tokens) {
            sum += result;
        }
    }
    Some(sum)
}

fn add_mul_concat(result: u64, lhs: u64, depth: usize, tokens: &[u64]) -> bool {
    if depth >= tokens.len() {
        return false;
    }

    let rhs = tokens[depth];
    let concat = concatenation(&[lhs, rhs]);
    if concat == result && depth == tokens.len() - 1 {
        return true;
    }
    if add_mul_concat(result, concat, depth + 1, tokens) {
        return true;
    }

    let product = lhs * rhs;
    if product == result && depth == tokens.len() - 1 {
        return true;
    }
    if add_mul_concat(result, product, depth + 1, tokens) {
        return true;
    }

    let sum = lhs + rhs;
    if sum == result && depth == tokens.len() - 1 {
        return true;
    }
    add_mul_concat(result, sum, depth + 1, tokens)
}

fn concatenation(nums: &[u64]) -> u64 {
    let mut result = nums[0];
    for &num in &nums[1..] {
        let mut digits = 1;
        let mut n = num;
        while n >= 10 {
            digits *= 10;
            n /= 10;
        }
        result = result * (digits * 10) + num;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    #[ignore = "inputs"]
    fn test_part_one_inputs() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1545311493300));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }

    #[test]
    #[ignore = "inputs"]
    fn test_part_two_inputs() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(169122112716571));
    }

    #[test]
    fn test_concatenation() {
        assert_eq!(concatenation(&[12, 345]), 12345);
        assert_eq!(concatenation(&[1, 2, 3]), 123);
        assert_eq!(concatenation(&[15, 6]), 156);
        assert_eq!(concatenation(&[15]), 15);
    }
}
