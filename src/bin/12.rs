use advent_of_code::repository::arrangement::{ConditionRecord, find_solutions, repeat_string, solve_sequence};
use itertools::Itertools;

advent_of_code::solution!(12);


pub fn part_one(input: &str) -> Option<u64> {
    // let condition_records = input.lines().map(|s| ConditionRecord::from(s)).collect_vec();
    // Some(find_solutions(condition_records))
    let mut result = 0;

    for line in input.lines() {
        let mut split = line.split(" ");
        let sequence = String::from(split.nth(0).unwrap());
        let damaged_springs: Vec<u32> = split.nth(0).unwrap().split(",").map(|s| s.parse().unwrap()).collect_vec();
        result += solve_sequence(sequence, damaged_springs);
    }

    // 8180
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.lines().map(|s| {
        let mut split = s.split(" ");
        format!("{} {}", repeat_string(split.nth(0).unwrap().to_string(), 5, "?"), repeat_string(split.nth(0).unwrap().to_string(), 5, ","))
    }).collect_vec();

    let mut result = 0;

    for line in lines {
        let mut split = line.split(" ");
        let sequence = String::from(split.nth(0).unwrap());
        let damaged_springs: Vec<u32> = split.nth(0).unwrap().split(",").map(|s| s.parse().unwrap()).collect_vec();
        result += solve_sequence(sequence, damaged_springs);
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
