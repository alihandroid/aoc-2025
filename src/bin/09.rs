advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let points = parse(input).collect::<Vec<_>>();

    points
        .iter().enumerate()
        .flat_map(|(i, &(x1, y1))| {
            points[i + 1..].iter()
                .map(move |&(x2, y2)| {
                    (x2.abs_diff(x1) + 1) * (y2.abs_diff(y1) + 1)
                })
        })
        .max()
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

fn parse(input: &str) -> impl Iterator<Item = (u64, u64)> {
    input.lines().map(|line| {
        let mut split = line.split(',');
        let (a, b) = match (split.next(), split.next()) {
            (Some(a), Some(b)) => (a, b),
            _ => panic!(),
        };
        match (a.parse::<u64>(), b.parse::<u64>()) {
            (Ok(a), Ok(b)) => (a, b),
            _ => panic!(),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
