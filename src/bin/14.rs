use std::str::FromStr;

use advent_of_code::repository::dish::Dish;

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    let dish = Dish::from_str(input).unwrap();
    Some(dish.tilt_north())
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}