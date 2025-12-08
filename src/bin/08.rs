use std::cmp::Reverse;
use std::collections::BinaryHeap;

advent_of_code::solution!(8);

const NUM_CONNECTIONS: usize = if cfg!(test) { 10 } else { 1000 };

struct Point {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Point {
    #[inline]
    fn new(x: i64, y: i64, z: i64) -> Point {
        Point { x, y, z }
    }

    /// Calculate squared distance to avoid expensive sqrt operation.
    /// Since sqrt preserves ordering, comparing squared distances gives the same result.
    #[inline]
    fn distance_squared(&self, p: &Point) -> i64 {
        let dx = self.x - p.x;
        let dy = self.y - p.y;
        let dz = self.z - p.z;
        dx * dx + dy * dy + dz * dz
    }
}

#[derive(Debug, Clone)]
pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
    components: usize,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            size: vec![1; n],
            components: n,
        }
    }

    #[inline]
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    #[inline]
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

        self.components -= 1;
        true
    }

    #[inline]
    pub fn size_of(&self, x: usize) -> usize {
        self.size[x]
    }

    #[inline]
    pub fn count_components(&self) -> usize {
        self.components
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
    let points = parse(input);
    let distances = calculate_distances_partial(&points, NUM_CONNECTIONS);
    let mut uf = UnionFind::new(points.len());

    for (_distance, i, j) in distances {
        uf.union(i, j);
    }

    let mut sizes: Vec<u64> = uf.roots().map(|root| uf.size_of(root) as u64).collect();

    sizes.select_nth_unstable_by(2, |a, b| b.cmp(a));

    let result = sizes[0] * sizes[1] * sizes[2];
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let points = parse(input);
    let distances = calculate_distances(&points);
    let mut uf = UnionFind::new(points.len());

    let mut heap = distances
        .into_iter()
        .map(Reverse)
        .collect::<BinaryHeap<_>>();

    while let Some(Reverse((_distance, i, j))) = heap.pop() {
        uf.union(i, j);
        if uf.count_components() == 1 {
            return Some((points[i].x * points[j].x) as u64);
        }
    }

    unreachable!()
}

fn calculate_distances(points: &[Point]) -> Vec<(i64, usize, usize)> {
    let mut distances = Vec::with_capacity((points.len() * (points.len() - 1)) / 2);

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let distance = points[i].distance_squared(&points[j]);
            distances.push((distance, i, j));
        }
    }

    distances
}

fn calculate_distances_partial(points: &[Point], limit: usize) -> Vec<(i64, usize, usize)> {
    let mut heap = BinaryHeap::with_capacity(limit + 1);

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let distance = points[i].distance_squared(&points[j]);

            if heap.len() < limit {
                heap.push((distance, i, j));
            } else if let Some(&(max_dist, _, _)) = heap.peek()
                && distance < max_dist
            {
                heap.pop();
                heap.push((distance, i, j));
            }
        }
    }

    let mut result = heap.into_vec();
    result.sort_unstable();
    result
}

fn parse(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let coords: Vec<i64> = line.split(',').map(|x| x.parse().unwrap()).collect();

            Point::new(coords[0], coords[1], coords[2])
        })
        .collect()
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
        assert_eq!(result, Some(25272));
    }
}
