use std::str::FromStr;

use advent_of_code::repository::platform::{Direction, Platform};

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    let mut platform = Platform::from_str(input).unwrap();
    platform.tilt(Direction::NORTH);
    Some(platform.total_load(Direction::NORTH))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut platform = Platform::from_str(input).unwrap();
    platform.cycle_n_times(1000000000);
    Some(platform.total_load(Direction::NORTH))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
