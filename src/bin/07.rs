advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.lines().collect::<Vec<_>>();
    let width = lines[0].len();
    let mut beams = vec![false; width];
    let start_column = lines[0].find('S').unwrap();
    beams[start_column] = true;
    let mut next_beams;
    let mut split_count = 0;

    for line in lines.into_iter().skip(1) {
        next_beams = vec![false; width];

        for (i, &ch) in line.as_bytes().into_iter().enumerate() {
            if !beams[i] {
                continue;
            }
            if ch == b'^' {
                next_beams[i - 1] = true;
                next_beams[i + 1] = true;
                split_count += 1;
            } else {
                next_beams[i] = true;
            }
        }

        beams = next_beams;
    }

    Some(split_count)
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
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
