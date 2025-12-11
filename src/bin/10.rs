use rayon::prelude::*;
use z3::ast::Int;
use z3::{Optimize, SatResult};

advent_of_code::solution!(10);

#[derive(Debug)]
struct ParsedInput {
    parsed_lines: Vec<ParsedLine>,
}

#[derive(Debug)]
struct ParsedLine {
    light_diagram: Vec<bool>,
    wiring_schematics: Vec<Vec<u64>>,
    joltage_requirements: Vec<u64>,
}

pub fn part_one(input: &str) -> Option<u64> {
    let parsed_input = parse(input);
    let result = parsed_input
        .parsed_lines
        .into_par_iter()
        .map(find_minimum_switches)
        .sum::<u32>();

    Some(result as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let parsed_input = parse(input);
    let result: u64 = parsed_input
        .parsed_lines
        .into_par_iter()
        .map(find_minimum_button_presses_z3)
        .sum();

    Some(result)
}

fn find_minimum_button_presses_z3(parsed_line: ParsedLine) -> u64 {
    let ParsedLine {
        wiring_schematics,
        joltage_requirements,
        ..
    } = parsed_line;

    let optimizer = Optimize::new();

    let num_buttons = wiring_schematics.len();

    let button_presses: Vec<Int> = (0..num_buttons)
        .map(|i| Int::new_const(format!("button_${i}")))
        .collect();

    for button_press in &button_presses {
        optimizer.assert(&button_press.ge(Int::from_i64(0)));
    }

    for (counter_idx, &target_joltage) in joltage_requirements.iter().enumerate() {
        let mut counter_sum_terms = Vec::new();

        for (button_idx, wiring) in wiring_schematics.iter().enumerate() {
            if wiring.contains(&(counter_idx as u64)) {
                counter_sum_terms.push(&button_presses[button_idx]);
            }
        }

        let counter_sum = if counter_sum_terms.is_empty() {
            Int::from_i64(0)
        } else if counter_sum_terms.len() == 1 {
            counter_sum_terms[0].clone()
        } else {
            Int::add(&counter_sum_terms.to_vec())
        };

        let target = Int::from_i64(target_joltage as i64);
        optimizer.assert(&counter_sum.eq(&target));
    }

    let total_presses = if button_presses.is_empty() {
        Int::from_i64(0)
    } else if button_presses.len() == 1 {
        button_presses[0].clone()
    } else {
        Int::add(&button_presses.iter().collect::<Vec<_>>())
    };

    optimizer.minimize(&total_presses);

    match optimizer.check(&[]) {
        SatResult::Sat => {
            let model = optimizer.get_model().unwrap();

            if let Some(total_value) = model.eval(&total_presses, true)
                && let Some(total) = total_value.as_i64()
            {
                return total as u64;
            }

            // Fallback: manually sum up individual button presses
            let mut total = 0i64;
            for button_press in &button_presses {
                if let Some(value) = model.eval(button_press, true)
                    && let Some(presses) = value.as_i64()
                {
                    total += presses;
                }
            }
            total as u64
        }
        SatResult::Unsat => {
            println!(
                "No solution found for joltage requirements: {:?}",
                joltage_requirements
            );
            0
        }
        SatResult::Unknown => {
            println!(
                "Z3 could not determine satisfiability for requirements: {:?}",
                joltage_requirements
            );
            0
        }
    }
}

fn find_minimum_switches(parsed_line: ParsedLine) -> u32 {
    let ParsedLine {
        light_diagram,
        wiring_schematics,
        ..
    } = parsed_line;

    (0..1_u64 << wiring_schematics.len())
        .filter_map(|bitmask| {
            if produces_target_diagram(bitmask, &wiring_schematics, &light_diagram) {
                Some(bitmask.count_ones())
            } else {
                None
            }
        })
        .min()
        .unwrap()
}

fn produces_target_diagram(
    bitmask: u64,
    wiring_schematics: &[Vec<u64>],
    target_diagram: &[bool],
) -> bool {
    let mut diagram = vec![false; target_diagram.len()];

    for (i, wiring_schematic) in wiring_schematics.iter().enumerate() {
        if (bitmask & (1 << i)) != 0 {
            toggle_lights(&mut diagram, wiring_schematic);
        }
    }

    diagram == target_diagram
}

fn toggle_lights(diagram: &mut [bool], light_indices: &[u64]) {
    for &light_index in light_indices {
        let index = light_index as usize;
        diagram[index] = !diagram[index];
    }
}

fn parse(input: &str) -> ParsedInput {
    let parsed_lines = input.lines().map(parse_line).collect();
    ParsedInput { parsed_lines }
}

fn parse_line(line: &str) -> ParsedLine {
    let light_diagram = parse_light_diagram(line);
    let wiring_schematics = parse_wiring_schematics(line);
    let joltage_requirements = parse_joltage_requirements(line);

    ParsedLine {
        light_diagram,
        wiring_schematics,
        joltage_requirements,
    }
}

fn parse_light_diagram(line: &str) -> Vec<bool> {
    let start = line.find('[').unwrap();
    let end = line.find(']').unwrap();

    line[start + 1..end]
        .bytes()
        .map(|byte| byte == b'#')
        .collect()
}

fn parse_wiring_schematics(line: &str) -> Vec<Vec<u64>> {
    let bracket_end = line.find(']').unwrap();
    let schematics_section = &line[bracket_end + 1..];

    schematics_section
        .split(')')
        .filter(|section| section.contains('('))
        .map(parse_single_schematic)
        .collect()
}

fn parse_single_schematic(section: &str) -> Vec<u64> {
    section
        .trim_start_matches([' ', '('])
        .split(',')
        .map(|number_str| number_str.trim().parse().unwrap())
        .collect()
}

fn parse_joltage_requirements(line: &str) -> Vec<u64> {
    let start = line.find('{').unwrap();
    let end = line.find('}').unwrap();

    line[start + 1..end]
        .split(',')
        .map(|number_str| number_str.trim().parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
