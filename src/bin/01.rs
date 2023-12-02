advent_of_code::solution!(1);
use std::str::FromStr;

use advent_of_code::repository::calibration::Callibration;


pub fn part_one(input: &str) -> Option<u32> {
    let mut result_sum = 0;

    let lines = advent_of_code::get_lines(input);
    for line in lines {
        let callibration = match Callibration::from_str(line) {
            Ok(c) => c,
            Err(_) => return None
        };
        let value = callibration.first_and_last_digit()?;
        result_sum += value.parse::<u32>().unwrap()
    }

    Some(result_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result_sum = 0;

    let lines = advent_of_code::get_lines(input);
    for line in lines {
        let callibration = match Callibration::from_str(line) {
            Ok(c) => c,
            Err(_) => return None
        };
        let value = callibration.first_and_last_digit_written()?;
        result_sum += value.parse::<u32>().unwrap()
    }
    
    Some(result_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(281));
    }
}
