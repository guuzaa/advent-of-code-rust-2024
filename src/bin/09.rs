advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let disk_map: Vec<u64> = input
        .chars()
        .filter_map(|c| c.to_digit(10).map(|x| x as u64))
        .collect();
    let cnt = disk_map.iter().sum::<u64>() as usize;
    let mut nums = vec![None; cnt];
    let mut cur = 0;
    for (i, &disk) in disk_map.iter().enumerate() {
        let end = cur + disk as usize;
        if i % 2 == 0 {
            nums[cur..end].fill(Some((i / 2) as u64));
        }
        cur = end;
    }

    let mut left = 0;
    let mut right = nums.len() - 1;
    while left < right {
        if nums[left].is_some() {
            left += 1;
            continue;
        }
        if nums[right].is_none() {
            right -= 1;
            continue;
        }

        nums[left] = nums[right];
        nums[right] = None;
        left += 1;
        right -= 1;
    }

    Some(
        nums.iter()
            .enumerate()
            .filter(|(_, x)| x.is_some())
            .map(|(i, x)| i as u64 * x.unwrap())
            .sum::<u64>(),
    )
}

#[derive(Debug)]
struct Chunk {
    uncompressed_index: usize,
    count: usize,
    file_id: usize,
}

pub fn part_two(input: &str) -> Option<u64> {
    let nums = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();
    let high_index: usize = nums.iter().sum();
    let uncompressed_reversed = nums
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &num_indices)| (nums.len() - 1 - i, num_indices));
    let reverse = uncompressed_reversed
        .scan(high_index, |base_index, (compressed_index, num_indices)| {
            *base_index -= num_indices;

            if compressed_index % 2 == 0 {
                Some(Some(Chunk {
                    uncompressed_index: *base_index,
                    count: num_indices,
                    file_id: compressed_index / 2,
                }))
            } else {
                Some(None)
            }
        })
        .flatten();

    // (uncompressed_index, space_count)
    let mut empties = nums
        .iter()
        .enumerate()
        .fold(
            (0, vec![]),
            |(mut uncompressed_index, mut empties), (compressed_index, &num_indices)| {
                if compressed_index % 2 != 0 {
                    empties.push((uncompressed_index, num_indices))
                }
                uncompressed_index += num_indices;
                (uncompressed_index, empties)
            },
        )
        .1;

    let mut moved_chunks: Vec<Chunk> = vec![];
    for chunk in reverse {
        let Some(empty) = empties
            .iter_mut()
            .find(|(i, empty_space)| chunk.count <= *empty_space && *i < chunk.uncompressed_index)
        else {
            continue;
        };

        moved_chunks.push(Chunk {
            uncompressed_index: empty.0,
            ..chunk
        });
        empty.0 += chunk.count;
        empty.1 -= chunk.count;
    }

    let mut base_index = 0;
    let mut sum = 0;
    for (compressed_index, num_indices) in nums.iter().enumerate() {
        let file_id = compressed_index / 2;
        if compressed_index % 2 == 0 && !moved_chunks.iter().any(|chunk| chunk.file_id == file_id) {
            let first = base_index;
            let last = base_index + *num_indices - 1;
            sum += file_id * (*num_indices * (first + last) / 2);
        }
        base_index += num_indices;
    }

    for chunk in moved_chunks.iter() {
        let first = chunk.uncompressed_index;
        let last = chunk.uncompressed_index + chunk.count - 1;
        sum += chunk.file_id * (chunk.count * (first + last) / 2);
    }
    Some(sum as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    #[ignore = "inputs"]
    fn test_part_one_inputs() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(6241633730082));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }

    #[test]
    #[ignore = "inputs"]
    fn test_part_two_inputs() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(6265268809555));
    }
}
