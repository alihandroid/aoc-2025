use std::cmp::PartialEq;

advent_of_code::solution!(1);

#[derive(Eq, PartialEq)]
enum Direction {
    Left,
    Right,
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut position = 50;
    let mut zero_count = 0;
    for (direction, number) in parse(input) {
        position = match direction {
            Direction::Left => position - number,
            Direction::Right => position + number,
        }
        .rem_euclid(100);

        if position == 0 {
            zero_count += 1;
        }
    }
    Some(zero_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

fn parse(input: &str) -> impl Iterator<Item = (Direction, i64)> + '_ {
    input.lines().filter(|line| !line.is_empty()).map(|line| {
        let direction = if line.as_bytes()[0] == b'L' {
            Direction::Left
        } else {
            Direction::Right
        };
        let number = line[1..].parse::<i64>().unwrap();
        (direction, number)
    })
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
