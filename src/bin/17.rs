advent_of_code::solution!(17);
use std::str::FromStr;

use advent_of_code::repository::crucible::{Map, shortest_path};

pub fn part_one(input: &str) -> Option<u32> {
    let map = Map::from_str(input).unwrap();
    shortest_path(map, 1, 3)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = Map::from_str(input).unwrap();
    shortest_path(map, 4, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }
}
