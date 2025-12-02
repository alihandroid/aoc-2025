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
    let mut sum = 0;
    for id_range in parse(input) {
        for id in id_range {
            let num_digits = id.ilog10() as u64 + 1;
            for &factor in FACTORS[num_digits as usize-1] {
                if is_invalid_n_repeats(id, num_digits, factor) {
                    sum += id;
                    break;
                }
            }
        }
    }
    Some(sum)
}

// we can pre-compute factors
const FACTORS: &[&[u64]] = &[
    &[],                // 1
    &[2],               // 2
    &[3],               // 3
    &[2, 4],            // 4
    &[5],               // 5
    &[2, 3, 6],         // 6
    &[7],               // 7
    &[2, 4, 8],         // 8
    &[3, 9],            // 9
    &[2, 5, 10],        // 10
    &[11],              // 11
    &[2, 3, 4, 6, 12],  // 12
    &[13],              // 13
    &[2, 7, 14],        // 14
    &[3, 5, 15],        // 15
    &[2, 4, 8, 16],     // 16
    &[17],              // 17
    &[2, 3, 6, 9, 18],  // 18
    &[19],              // 19
    &[2, 4, 5, 10, 20], // 20
];

fn is_invalid(id: u64) -> bool {
    let num_digits = id.ilog10() as u64 + 1;
    let half = num_digits / 2;
    let half_10 = 10_u64.pow(half as u32);
    let first_half = id / half_10;
    let second_half = id % half_10;
    first_half == second_half
}

fn is_invalid_n_repeats(mut id: u64, num_digits: u64, n: u64) -> bool {
    let part = num_digits / n;
    let part_10 = 10_u64.pow(part as u32);

    let mut prev_remainder = id % part_10;
    id /= part_10;
    let mut remainder;
    for _ in 1..n {
        remainder = id % part_10;
        if remainder != prev_remainder {
            return false;
        }
        id /= part_10;
        prev_remainder = remainder;
    }
    true
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
        assert_eq!(result, Some(4174379265));
    }
}
