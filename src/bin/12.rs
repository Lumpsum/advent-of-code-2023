use std::str::FromStr;

use advent_of_code::repository::arrangement::{ConditionRecord, find_solutions};
use itertools::Itertools;

advent_of_code::solution!(12);


pub fn part_one(input: &str) -> Option<u32> {
    let condition_records = input.lines().map(|s| ConditionRecord::from(s)).collect_vec();
    find_solutions(condition_records);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
