use std::cmp::PartialEq;

advent_of_code::solution!(1);

const MAX_NUM: u64 = 100;

#[derive(Eq, PartialEq, Debug)]
enum Direction {
    Left,
    Right,
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut position = 50;
    let mut zero_count = 0;
    for (direction, number) in parse(input) {
        position = match direction {
            Direction::Left => position - number as i64,
            Direction::Right => position + number as i64,
        }
        .rem_euclid(MAX_NUM as i64);

        if position == 0 {
            zero_count += 1;
        }
    }
    Some(zero_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut position = 50;
    let mut zero_count = 0;
    for (direction, number) in parse(input) {
        let full_turns = if position == 0 {
            number
        } else {
            match direction {
                Direction::Left => number + MAX_NUM - position as u64,
                Direction::Right => number + position as u64,
            }
        } / MAX_NUM;

        position = match direction {
            Direction::Left => position - number as i64,
            Direction::Right => position + number as i64,
        }
        .rem_euclid(MAX_NUM as i64);

        zero_count += full_turns;
    }
    Some(zero_count)
}

fn parse(input: &str) -> impl Iterator<Item = (Direction, u64)> + '_ {
    input[..input.len() - 2].lines().map(|line| {
        let direction = if line.as_bytes()[0] == b'L' {
            Direction::Left
        } else {
            Direction::Right
        };
        let number = line[1..].parse::<u64>().unwrap();
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
        assert_eq!(result, Some(6));
    }
}
