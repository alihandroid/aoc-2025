advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum = 0;
    for id_range in parse(input) {
        for id in id_range {
            if is_invalid(id) {
                sum += id;
            }
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

fn is_invalid(id: u64) -> bool {
    let num_digits = id.ilog10() as u64 + 1;
    let half = num_digits / 2;
    let half_10 = 10_u64.pow(half as u32);
    let first_half = id / half_10;
    let second_half = id % half_10;
    first_half == second_half
}

fn parse(input: &str) -> impl Iterator<Item = impl Iterator<Item = u64> + '_> + '_ {
    input.trim_end().split(',').map(|line| {
        let dash_index = line.find('-').unwrap();
        let first = line[0..dash_index].parse::<u64>().unwrap();
        let second = line[dash_index + 1..].parse::<u64>().unwrap();
        first..=second
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
