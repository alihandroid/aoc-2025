advent_of_code::solution!(4);

struct TpMap {
    has_tp: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

pub fn part_one(input: &str) -> Option<u64> {
    let map = parse(input);
    let mut num_accessible_rolls = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            if !map.has_tp[y][x] {
                continue;
            }

            if is_accessible(&map, x, y) {
                num_accessible_rolls += 1;
            }
        }
    }
    Some(num_accessible_rolls)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut map = parse(input);
    let mut num_accessible_rolls = 0;

    loop {
        let mut positions_to_remove = Vec::new();

        for y in 0..map.height {
            for x in 0..map.width {
                if !map.has_tp[y][x] {
                    continue;
                }

                if is_accessible(&map, x, y) {
                    positions_to_remove.push((x, y));
                }
            }
        }

        if positions_to_remove.is_empty() {
            break;
        }

        for (x, y) in positions_to_remove {
            map.has_tp[y][x] = false;
            num_accessible_rolls += 1;
        }
    }

    Some(num_accessible_rolls)
}

fn is_accessible(map: &TpMap, x: usize, y: usize) -> bool {
    let mut count = 0;

    let start_y = y.saturating_sub(1);
    let end_y = (y + 1).min(map.height - 1);
    let start_x = x.saturating_sub(1);
    let end_x = (x + 1).min(map.width - 1);

    for new_y in start_y..=end_y {
        for new_x in start_x..=end_x {
            if new_y == y && new_x == x || !map.has_tp[new_y][new_x] {
                continue;
            }
            count += 1;
            if count >= 4 {
                return false;
            }
        }
    }
    true
}

fn parse(input: &str) -> TpMap {
    let has_tp = input
        .lines()
        .map(|line| line.bytes().map(|ch| ch == b'@').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let width = has_tp[0].len();
    let height = has_tp.len();
    TpMap {
        has_tp,
        width,
        height,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
