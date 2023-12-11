advent_of_code::solution!(11);
use std::str::FromStr;

use advent_of_code::repository::galaxy::Space;


pub fn part_one(input: &str) -> Option<i64> {
    let space = Space::from_str(input).unwrap();
    Some(space.sum_shortest_paths(1))
}

pub fn part_two(input: &str) -> Option<i64> {
    let space = Space::from_str(input).unwrap();
    Some(space.sum_shortest_paths(999999))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8410));
    }
}
