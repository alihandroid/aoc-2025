advent_of_code::solution!(12);

#[derive(Debug)]
struct Shape {
    cells: Vec<Vec<bool>>,
    width: u32,
    height: u32,
}

#[derive(Debug)]
struct RegionRequest {
    width: u32,
    height: u32,
    quantities: Vec<u32>,
}

#[derive(Debug)]
struct ParsedInput {
    shapes: Vec<Shape>,
    regions: Vec<RegionRequest>,
}

pub fn part_one(input: &str) -> Option<u64> {
    let ParsedInput { shapes, regions } = parse(input);

    let result = regions
        .into_iter()
        .filter(|x| can_presents_fit(x, &shapes))
        .count() as u64;
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

fn can_presents_fit(region: &RegionRequest, shapes: &[Shape]) -> bool {
    (region.width/3)*(region.height/3) >= region.quantities.iter().sum()
}


fn parse(input: &str) -> ParsedInput {
    let mut shapes = input.split("\n\n").collect::<Vec<_>>();
    let regions = shapes.pop().unwrap();

    ParsedInput {
        shapes: parse_shapes(shapes),
        regions: parse_regions(regions),
    }
}

fn parse_shapes(shapes_str: Vec<&str>) -> Vec<Shape> {
    shapes_str
        .into_iter()
        .map(|shape| {
            let cells = shape
                .lines()
                .skip(1)
                .map(|line| line.bytes().map(|b| b == b'#').collect::<Vec<_>>())
                .collect::<Vec<_>>();
            Shape {
                width: cells.len() as u32,
                height: cells.first().unwrap().len() as u32,
                cells,
            }
        })
        .collect::<Vec<_>>()
}

fn parse_regions(regions_str: &str) -> Vec<RegionRequest> {
    regions_str
        .lines()
        .map(|line| {
            let (region, quantities) = line.split_once(": ").unwrap();
            let (width, height) = region.split_once('x').unwrap();

            let width = width.parse::<u32>().unwrap();
            let height = height.parse::<u32>().unwrap();
            let quantities = quantities
                .split(" ")
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            RegionRequest {
                width,
                height,
                quantities,
            }
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
