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

    let mut new_map = TpMap {
        has_tp: map.has_tp.clone(),
        width: map.width,
        height: map.height,
    };

    loop {
        let mut has_removed_tp = false;

        for y in 0..map.height {
            for x in 0..map.width {
                if !map.has_tp[y][x] {
                    continue;
                }

                if is_accessible(&map, x, y) {
                    num_accessible_rolls += 1;
                    new_map.has_tp[y][x] = false;
                    has_removed_tp = true;
                }
            }
        }

        if !has_removed_tp {
            break;
        }

        map.has_tp = new_map.has_tp.clone();
    }

    Some(num_accessible_rolls)
}

fn is_accessible(map: &TpMap, x: usize, y: usize) -> bool {
    let mut count = 0;
    for d_y in -1..=1 {
        for d_x in -1..=1 {
            if d_y == 0 && d_x == 0 {
                continue;
            }

            let new_y = y as i32 + d_y;
            let new_x = x as i32 + d_x;

            if new_y < 0 || new_y >= map.height as i32 || new_x < 0 || new_x >= map.width as i32 {
                continue;
            }

            if map.has_tp[new_y as usize][new_x as usize] {
                count += 1;
                if count >= 4 {
                    return false;
                }
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
