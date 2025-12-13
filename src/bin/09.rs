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
    let points = parse(input).collect::<Vec<_>>();
    if points.len() < 2 {
        return None;
    }

    // Coordinate compression
    let mut uniq_x: Vec<u64> = points.iter().map(|&(x, _)| x).collect();
    uniq_x.sort_unstable();
    uniq_x.dedup();

    let mut uniq_y: Vec<u64> = points.iter().map(|&(_, y)| y).collect();
    uniq_y.sort_unstable();
    uniq_y.dedup();

    let w = uniq_x.len();
    let h = uniq_y.len();
    if w == 0 || h == 0 {
        return None;
    }

    let mut grid: Vec<Vec<u8>> = vec![vec![b'.'; w]; h];

    let mut z_points: Vec<(usize, usize)> = Vec::with_capacity(points.len());
    for &(x, y) in &points {
        let zx = index_of(&uniq_x, x);
        let zy = index_of(&uniq_y, y);
        grid[zy][zx] = b'#';
        z_points.push((zx, zy));
    }

    for i in 0..z_points.len() {
        let (ax, ay) = z_points[i];
        let (bx, by) = z_points[(i + 1) % z_points.len()];

        if ax == bx {
            let (y0, y1) = if ay <= by { (ay, by) } else { (by, ay) };
            for y in y0..=y1 {
                grid[y][ax] = b'#';
            }
        } else if ay == by {
            let (x0, x1) = if ax <= bx { (ax, bx) } else { (bx, ax) };
            for x in x0..=x1 {
                grid[ay][x] = b'#';
            }
        } else {
            // No diagonal edges
            return None;
        }
    }

    let inside = get_inside_point(&grid)?;
    flood_fill(&mut grid, inside);

    let mut best: u64 = 0;
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let a = points[i];
            let b = points[j];

            if is_enclosed(a, b, &grid, &uniq_x, &uniq_y) {
                let area = (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1);
                best = best.max(area);
            }
        }
    }

    Some(best)
}

fn index_of(sorted_uniq: &[u64], v: u64) -> usize {
    sorted_uniq
        .binary_search(&v)
        .expect("value must exist in its own uniq array")
}

fn is_enclosed(
    a: (u64, u64),
    b: (u64, u64),
    grid: &[Vec<u8>],
    uniq_x: &[u64],
    uniq_y: &[u64],
) -> bool {
    let mut x1 = index_of(uniq_x, a.0);
    let mut x2 = index_of(uniq_x, b.0);
    if x1 > x2 {
        std::mem::swap(&mut x1, &mut x2);
    }

    let mut y1 = index_of(uniq_y, a.1);
    let mut y2 = index_of(uniq_y, b.1);
    if y1 > y2 {
        std::mem::swap(&mut y1, &mut y2);
    }

    for x in x1..=x2 {
        if grid[y1][x] == b'.' || grid[y2][x] == b'.' {
            return false;
        }
    }
    for y in y1..=y2 {
        if grid[y][x1] == b'.' || grid[y][x2] == b'.' {
            return false;
        }
    }
    true
}

fn flood_fill(grid: &mut [Vec<u8>], start: (usize, usize)) {
    let mut stack = vec![start];
    let dirs: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    while let Some((x, y)) = stack.pop() {
        if grid[y][x] != b'.' {
            continue;
        }
        grid[y][x] = b'X';

        for (dx, dy) in dirs {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if ny >= 0
                && (ny as usize) < grid.len()
                && nx >= 0
                && (nx as usize) < grid[0].len()
            {
                let (nxu, nyu) = (nx as usize, ny as usize);
                if grid[nyu][nxu] == b'.' {
                    stack.push((nxu, nyu));
                }
            }
        }
    }
}

fn get_inside_point(grid: &[Vec<u8>]) -> Option<(usize, usize)> {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] != b'.' {
                continue;
            }

            let mut hits_left = 0usize;
            let mut prev = b'.';

            for i in (0..=x).rev() {
                let cur = grid[y][i];
                if cur != prev {
                    hits_left += 1;
                }
                prev = cur;
            }

            if hits_left % 2 == 1 {
                return Some((x, y));
            }
        }
    }
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
        assert_eq!(result, Some(24));
    }
}
