advent_of_code::solution!(5);
use std::str::FromStr;

use advent_of_code::repository::almanac;


pub fn part_one(input: &str) -> Option<u32> {
    let mut almanac = almanac::Almanac::from_str(input).unwrap();
    Some(almanac.calculate_seed_locations())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut almanac = almanac::Almanac::from_str(input).unwrap();
    Some(almanac.calculate_seed_locations())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
