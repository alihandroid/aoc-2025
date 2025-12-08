use std::hint::unreachable_unchecked;

advent_of_code::solution!(8);

const NUM_CONNECTIONS: usize = if cfg!(test) { 10 } else { 1000 };

struct Point {
    x: u64,
    y: u64,
    z: u64,
}

impl Point {
    fn new(x: u64, y: u64, z: u64) -> Point {
        Point { x, y, z }
    }

    /// Calculate squared distance to avoid expensive sqrt operation.
    /// Since sqrt preserves ordering, comparing squared distances gives the same result.
    fn distance_squared(&self, p: &Point) -> u64 {
        let dx = self.x.abs_diff(p.x);
        let dy = self.y.abs_diff(p.y);
        let dz = self.z.abs_diff(p.z);
        dx * dx + dy * dy + dz * dz
    }
}

#[derive(Debug, Clone)]
pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            size: vec![1; n],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        // Union by rank for better performance
        match self.rank[root_x].cmp(&self.rank[root_y]) {
            std::cmp::Ordering::Less => {
                self.parent[root_x] = root_y;
                self.size[root_y] += self.size[root_x];
            }
            std::cmp::Ordering::Greater => {
                self.parent[root_y] = root_x;
                self.size[root_x] += self.size[root_y];
            }
            std::cmp::Ordering::Equal => {
                self.parent[root_y] = root_x;
                self.size[root_x] += self.size[root_y];
                self.rank[root_x] += 1;
            }
        }

        true
    }

    pub fn size_of(&self, x: usize) -> usize {
        self.size[x]
    }

    pub fn roots(&self) -> impl Iterator<Item = usize> {
        self.parent
            .iter()
            .enumerate()
            .filter(|(i, p)| *p == i)
            .map(|(i, _)| i)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let points = parse(input).collect::<Vec<_>>();

    let mut distances = Vec::new();
    // calculate distances between every unordered pair of points
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let distance = points[i].distance_squared(&points[j]);
            distances.push((distance, i, j));
        }
    }
    distances.sort();

    let mut uf = UnionFind::new(points.len());

    for (_distance, i, j) in distances.iter().take(NUM_CONNECTIONS) {
        uf.union(*i, *j);
    }

    let mut sizes = uf.roots().map(|root| uf.size_of(root) as u64).collect::<Vec<_>>();

    sizes.sort_unstable_by(|a,b|b.cmp(a));

    let result = sizes[0] * sizes[1] * sizes[2];
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

fn parse(input: &str) -> impl Iterator<Item = Point> {
    input.lines().map(|line| {
        let [x, y, z, ..] = line
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect::<Vec<u64>>()[..]
        else {
            // SAFETY: nuh-uh 🗿
            unsafe {
                unreachable_unchecked();
            }
        };

        Point::new(x, y, z)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
