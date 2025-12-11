use std::collections::HashMap;

advent_of_code::solution!(11);

struct Graph {
    labels: HashMap<String, usize>,
    edges: HashMap<usize, Vec<usize>>,
}

pub fn part_one(input: &str) -> Option<u64> {
    let Graph { labels, edges } = parse(input);

    let start_index = *labels.get("you").unwrap();
    let end_index = *labels.get("out").unwrap();

    let mut memo = HashMap::new();
    let mut visited = vec![false; labels.len()];
    let result = count_paths(
        start_index,
        end_index,
        usize::MAX,
        usize::MAX,
        &mut visited,
        false,
        false,
        &edges,
        &mut memo,
    );
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let Graph { labels, edges } = parse(input);

    let start_index = *labels.get("svr")?;
    let end_index = *labels.get("out")?;
    let dac_index = *labels.get("dac")?;
    let fft_index = *labels.get("fft")?;

    let mut memo = HashMap::new();
    let mut visited = vec![false; labels.len()];
    let result = count_paths(
        start_index,
        end_index,
        dac_index,
        fft_index,
        &mut visited,
        false,
        false,
        &edges,
        &mut memo,
    );
    Some(result)
}

fn count_paths(
    current: usize,
    end: usize,
    dac: usize,
    fft: usize,
    visited: &mut Vec<bool>,
    has_dac: bool,
    has_fft: bool,
    edges: &HashMap<usize, Vec<usize>>,
    memo: &mut HashMap<(usize, bool, bool), u64>,
) -> u64 {
    if current == end {
        return if has_dac && has_fft { 1 } else { 0 };
    }

    // Check memo (but only if we haven't visited this node yet in current path)
    if !visited[current] {
        let key = (current, has_dac, has_fft);
        if let Some(&cached) = memo.get(&key) {
            return cached;
        }
    }

    if visited[current] {
        return 0;
    }

    visited[current] = true;

    let new_has_dac = has_dac || (current == dac);
    let new_has_fft = has_fft || (current == fft);

    let mut total = 0;
    if let Some(neighbors) = edges.get(&current) {
        for &next in neighbors {
            total += count_paths(
                next,
                end,
                dac,
                fft,
                visited,
                new_has_dac,
                new_has_fft,
                edges,
                memo,
            );
        }
    }

    visited[current] = false;

    // Only memoize if this was a "clean" call (no cycles)
    if !visited[current] {
        let key = (current, has_dac, has_fft);
        memo.insert(key, total);
    }

    total
}

fn parse(input: &str) -> Graph {
    let mut labels = HashMap::new();

    let mut get_or_create_index = |label: &str| {
        if let Some(&index) = labels.get(label) {
            index
        } else {
            let index = labels.len();
            labels.insert(label.to_string(), index);
            index
        }
    };

    let edges = input
        .lines()
        .map(|line| {
            let (node, connections_str) = line.split_once(':').unwrap();

            let node_index = get_or_create_index(node);

            (
                node_index,
                connections_str
                    .split_whitespace()
                    .map(|connection| get_or_create_index(connection))
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<HashMap<_, _>>();

    Graph { labels, edges }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }
}
