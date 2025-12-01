advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut position = 50;
    let mut zero_count = 0;
    for line in input.split('\n') {
        if (line.len() == 0) {
            continue;
        }
        let first_char = line.as_bytes()[0];
        let number = line[1..].parse::<i64>().unwrap();

        if first_char == b'L' {
            position = (position - number).rem_euclid(100);
        } else {
            position = (position + number).rem_euclid(100);
        }

        if position == 0 {
            zero_count += 1;
        }
    }
    Some(zero_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
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
        assert_eq!(result, None);
    }
}
