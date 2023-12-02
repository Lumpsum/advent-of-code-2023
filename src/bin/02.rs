advent_of_code::solution!(2);
use std::str::FromStr;

use advent_of_code::repository::cubes;

pub fn part_one(input: &str) -> Option<u32> {
    let mut result_sum: u32 = 0;

    let lines = advent_of_code::get_lines(input);
    for line in lines {
        let mut cube = cubes::Game::from_str(line).unwrap();
        if cube.validate_all_rounds(cubes::CubeSet{ blue: 14, red: 12, green: 13 }) {
            result_sum += cube.id
        }
    }
    Some(result_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result_sum: u32 = 0;

    let lines = advent_of_code::get_lines(input);
    for line in lines {
        let cube = cubes::Game::from_str(line).unwrap();
        result_sum += cube.minimum_set().power()
    }
    Some(result_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
