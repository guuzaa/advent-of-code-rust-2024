advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut sum = 0;
    for cap in re.captures_iter(input) {
        if let (Ok(x), Ok(y)) = (cap[1].parse::<u32>(), cap[2].parse::<u32>()) {
            sum += x * y;
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    // Regex for mul instructions and do/don't controls
    let mul_re = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let do_re = regex::Regex::new(r"do\(\)").unwrap();
    let dont_re = regex::Regex::new(r"don't\(\)").unwrap();

    let mut sum = 0;
    let mut enabled = true; // mul instructions are enabled by default
    let mut pos = 0;
    let max_pos = input.len() + 1;

    let mut mul_match = mul_re.find_at(input, pos);
    let mut do_match = do_re.find_at(input, pos);
    let mut dont_match = dont_re.find_at(input, pos);

    let mut mul_pos = mul_match.map(|m| m.end()).unwrap_or(max_pos);
    let mut do_pos = do_match.map(|m| m.end()).unwrap_or(max_pos);
    let mut dont_pos = dont_match.map(|m| m.end()).unwrap_or(max_pos);
    let mut min_pos = mul_pos.min(do_pos).min(dont_pos);

    while pos < input.len() && min_pos < max_pos {
        if min_pos == mul_pos {
            let m = mul_match.unwrap();
            if enabled {
                if let Some(cap) = mul_re.captures(&input[m.start()..m.end()]) {
                    if let (Ok(x), Ok(y)) = (cap[1].parse::<u32>(), cap[2].parse::<u32>()) {
                        sum += x * y;
                    }
                }
            }
            mul_match = mul_re.find_at(input, mul_pos);
            mul_pos = mul_match.map(|m| m.end()).unwrap_or(max_pos);
        } else if min_pos == do_pos {
            enabled = true;
            do_match = do_re.find_at(input, do_pos);
            do_pos = do_match.map(|m| m.end()).unwrap_or(max_pos);
        } else {
            enabled = false;
            dont_match = dont_re.find_at(input, dont_pos);
            dont_pos = dont_match.map(|m| m.end()).unwrap_or(max_pos);
        }
        pos = min_pos;
        min_pos = mul_pos.min(do_pos).min(dont_pos);
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
