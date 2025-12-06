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
    } = parse(input);

    let result: u64 = (0..numbers.len())
        .map(|i| operations[i].run(&numbers[i]))
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

fn parse(input: &str) -> ParsedInput {
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
        .map(|i| {
            numbers
                .iter()
                .map(|arr| arr[i])
                .collect::<Vec<_>>()
        })
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
        assert_eq!(result, None);
    }
}
