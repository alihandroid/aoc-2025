use std::str::FromStr;

advent_of_code::solution!(6);

#[derive(Debug)]
enum Operation {
    Add,
    Mul,
}

impl Operation {
    fn run(&self, nums: &[u64]) -> u64 {
        match self {
            Operation::Add => nums.iter().sum(),
            Operation::Mul => nums.iter().product(),
        }
    }
}

impl FromStr for Operation {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operation::Add),
            "*" => Ok(Operation::Mul),
            _ => unreachable!(),
        }
    }
}

struct ParsedInput {
    numbers: Vec<Vec<u64>>,
    operations: Vec<Operation>,
}

pub fn part_one(input: &str) -> Option<u64> {
    let ParsedInput {
        numbers,
        operations,
    } = parse_part_one(input);

    let result = calculate(numbers, operations);

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ParsedInput {
        numbers,
        operations,
    } = parse_part_two(input);

    let result = calculate(numbers, operations);

    Some(result)
}

fn calculate(numbers: Vec<Vec<u64>>, operations: Vec<Operation>) -> u64 {
    numbers
        .iter()
        .zip(operations)
        .map(|(nums, op)| op.run(nums))
        .sum()
}

fn parse_part_one(input: &str) -> ParsedInput {
    let mut lines = input.lines().collect::<Vec<_>>();
    let operations_line = lines.pop().unwrap();

    let numbers = lines
        .into_iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // transpose
    let numbers = (0..numbers[0].len())
        .map(|i| numbers.iter().map(|arr| arr[i]).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let operations = operations_line
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();

    ParsedInput {
        numbers,
        operations,
    }
}

fn parse_part_two(input: &str) -> ParsedInput {
    let mut lines = input.lines().collect::<Vec<_>>();
    let operations_line = lines.pop().unwrap();

    // transpose
    let numbers = (0..lines[0].len())
        .map(|i| {
            let s = lines
                .iter()
                .map(|arr| arr.as_bytes()[i] as char)
                .collect::<String>();
            s.trim().to_owned()
        })
        .collect::<Vec<_>>();

    // parse
    let numbers = numbers
        .split(|s| s.len() == 0)
        .map(|x| {
            x.into_iter()
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // same as part 1
    let operations = operations_line
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();

    ParsedInput {
        numbers,
        operations,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
