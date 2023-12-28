use advent_of_code::repository::pulse::{get_modules_and_edges, get_pulses, get_first_pulse, lcm_vec};
use itertools::Itertools;

advent_of_code::solution!(20);

pub fn part_one<'a>(input: &'a str) -> Option<u32> {
    let (modules, _) = get_modules_and_edges(input);
    Some(get_pulses(1000, modules))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut modules, edges) = get_modules_and_edges(input);

    let mut input: &str = "";
    for edge in edges {
        if edge.1 == "rx" {
            input = edge.0;
            break
        }
    }

    let inputs = modules.get_mut(input).unwrap().get_inputs().unwrap();
    let first_pulses = get_first_pulse(inputs.keys().map(|&s| s).collect_vec(), modules);
    
    // println!("{:?}", inputs.keys().collect_vec());
    // {"rk": Low, "zf": Low, "qx": Low, "cd": Low}

    // press_button(15000, modules);

    Some(lcm_vec(first_pulses))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11687500));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }
}
