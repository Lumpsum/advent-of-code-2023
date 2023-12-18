use std::str::FromStr;

use advent_of_code::repository::mirror::Mirror;

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u32> {
    for mirror in input.split("\r\n\r\n") {
        Mirror::from_str(mirror);
    }
    None
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
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
