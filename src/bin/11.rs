use std::collections::HashMap;

advent_of_code::solution!(11);

struct Graph {
    labels: HashMap<String, usize>,
    edges: HashMap<usize, Vec<usize>>,
}

pub fn part_one(input: &str) -> Option<u64> {
    let Graph {labels, edges } = parse(input);

    let start_index = *labels.get("you").unwrap();
    let end_index = *labels.get("out").unwrap();

    let mut visited = vec![false; labels.len()];

    let mut stack = Vec::new();
    let mut num_paths = 0;

    stack.push((start_index, false));

    while let Some((current, is_backtrack)) = stack.pop() {
        if is_backtrack {
            visited[current] = false;
            continue;
        }

        if current == end_index {
            num_paths += 1;
            continue;
        }

        if visited[current] {
            continue;
        }

        visited[current] = true;

        stack.push((current, true));

        if let Some(neighbors) = edges.get(&current) {
            for &next in neighbors.iter().rev() {
                if !visited[next] {
                    stack.push((next, false));
                }
            }
        }
    }
    Some(num_paths)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
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
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
