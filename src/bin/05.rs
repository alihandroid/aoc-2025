advent_of_code::solution!(5);

struct ParsedInput {
    ranges: Vec<(u64, u64)>,
    ids: Vec<u64>,
}

pub fn part_one(input: &str) -> Option<u64> {
    let ParsedInput { mut ranges, ids } = parse(input);
    let merged_ranges = merge_ranges(&mut ranges);
    let result = ids
        .into_iter()
        .filter(|id| is_fresh(&merged_ranges, id))
        .count();
    Some(result as u64)
}
pub fn part_two(input: &str) -> Option<u64> {
    let ParsedInput { mut ranges, .. } = parse(input);

    let result = merge_ranges(&mut ranges)
        .iter()
        .map(|&(start, end)| end - start + 1)
        .sum();
    Some(result)
}

fn merge_ranges(ranges: &mut [(u64, u64)]) -> Vec<(u64, u64)> {
    ranges.sort();
    let mut ranges_iter = ranges.iter();
    let mut merged_ranges: Vec<(u64, u64)> = vec![*ranges_iter.next().unwrap()];
    for &(start, end) in ranges_iter {
        let (prev_start, prev_end) = *merged_ranges.last().unwrap();
        if start >= prev_start && start <= prev_end {
            merged_ranges.pop();
            merged_ranges.push((prev_start, prev_end.max(end)));
        } else {
            merged_ranges.push((start, end));
        }
    }
    merged_ranges
}

fn is_fresh(ranges: &[(u64, u64)], ids: &u64) -> bool {
    ranges
        .iter()
        .any(|(start, end)| (start..=end).contains(&ids))
}

fn parse(input: &str) -> ParsedInput {
    let input = input.replace("\r\n", "\n");
    let sections: Vec<&str> = input.split("\n\n").collect();

    let [ranges_str, ids_str] = sections[..] else {
        panic!("invalid input");
    };

    let ranges = ranges_str
        .lines()
        .map(|line| {
            let parts: Vec<u64> = line
                .split('-')
                .map(|s| s.parse().expect("Invalid number in range"))
                .collect();
            (parts[0], parts[1])
        })
        .collect();

    let ids = ids_str
        .lines()
        .map(|line| line.parse().expect("Invalid ID number"))
        .collect();

    ParsedInput { ranges, ids }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
