advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    Some(parse(input).map(find_max_joltage).sum())
}

fn find_max_joltage(s: &str) -> u64 {
    let bytes = s.as_bytes();
    let mut tens = 0;
    let mut tens_index= 0;

    for (i,b) in bytes[..bytes.len() - 1].iter().enumerate() {
        let num = b - b'0';
        if num > tens {
            tens = num;
            tens_index = i;
        }
    }

    let mut ones = 0;
    for b in bytes[tens_index+1..].iter() {
        let num = b - b'0';
        if num > ones {
            ones = num;
        }
    }

    (tens * 10 + ones) as u64
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

fn parse(input: &str) -> impl Iterator<Item = &str> + '_ {
    input.lines()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
