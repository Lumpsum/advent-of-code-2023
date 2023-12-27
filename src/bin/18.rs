use std::str::FromStr;

use advent_of_code::repository::lagoon::DigPlan;

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<u32> {
    let dig_plan = DigPlan::from_str(input).unwrap();
    Some(dig_plan.calculate_area() + dig_plan.calculate_perimeter() / 2 + 1)
}

pub fn part_two(input: &str) -> Option<i64> {
    let dig_plan = DigPlan::from_str(input).unwrap();
    Some(dig_plan.rgb_steps())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
