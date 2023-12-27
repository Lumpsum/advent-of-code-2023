use std::collections::VecDeque;

use advent_of_code::repository::workflow::{get_next_destination, Workflows, Range};
use itertools::Itertools;

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u32> {
    let mut result = 0;
    let mut splits = input.split("\r\n\r\n");

    let workflows = Workflows::from(splits.nth(0).unwrap());

    let parts = splits.nth(0).unwrap().lines().map(|s| {
        let mut chars = s.chars();
        chars.next();
        chars.next_back();
        chars.collect::<String>().split(",").map(|s| {
            let (_, digit) = s.rsplit_once("=").unwrap();
            digit.parse::<u32>().unwrap()
    }).collect_vec()
    }).collect_vec();
    
    for part in parts {
        let mut q = VecDeque::new();
        q.push_front("in");

        while let Some(v) = q.pop_front() {
            let current_workflow = workflows.0.get(v).unwrap();

            let next_destination = get_next_destination(part.clone(), current_workflow);
            match next_destination {
                "R" => continue,
                "A" => result += part.iter().sum::<u32>(),
                v => q.push_front(v)
            }
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut result = 0;
    let mut splits = input.split("\r\n\r\n");

    let workflows = Workflows::from(splits.nth(0).unwrap());
    let mut q = VecDeque::new();
    q.push_front((vec![Range{min: 1, max: 4000}; 4], "in"));

    while let Some(v) = q.pop_front() {
        match v.1 {
            "A" => {result += v.0.iter().map(|r| r.max - r.min + 1).collect_vec().iter().product::<u64>(); continue},
            "R" => continue,
            _ => ()
        }

        let workflow = workflows.0.get(v.1).unwrap();
        let mut current_ranges = v.0.clone();

        for step in workflow.steps.iter() {
            match step.ordering {
                std::cmp::Ordering::Less => {
                    match (current_ranges[step.part].min < step.value, current_ranges[step.part].max < step.value) {
                        (true, true) => {
                            q.push_back((current_ranges.clone(), step.destination));
                            break;
                        },
                        (true, false) => {
                            let mut new_value = current_ranges.clone();
                            current_ranges[step.part].min = step.value;
                            new_value[step.part].max = step.value - 1;
                            q.push_back((new_value, step.destination));
                        },
                        (false, true) => todo!(),
                        (false, false) => continue,
                    }
                },
                std::cmp::Ordering::Greater => {
                    match (current_ranges[step.part].min > step.value, current_ranges[step.part].max > step.value) {
                        (true, true) => {
                            q.push_back((current_ranges.clone(), step.destination));
                            break;
                        },
                        (true, false) => todo!(),
                        (false, true) => {
                            let mut new_value = current_ranges.clone();
                            current_ranges[step.part].max = step.value;
                            new_value[step.part].min = step.value + 1;
                            q.push_back((new_value, step.destination))
                        },
                        (false, false) => continue,
                    }
                },
                std::cmp::Ordering::Equal => todo!(),
            }
        }

        q.push_back((current_ranges, workflow.destination));
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}
