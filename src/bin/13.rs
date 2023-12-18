use std::str::FromStr;

use advent_of_code::repository::mirror::Mirror;

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u32> {
    let mut result = 0;
    for mirror in input.split("\n\n") {
        let mirror = Mirror::from_str(mirror).unwrap();
        result += mirror.find_mirror();
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result = 0;
    for mirror in input.split("\n\n") {
        let mirror = Mirror::from_str(mirror).unwrap();
        let new_value = mirror.find_mirror_smudge();
        result += new_value;
    }
    Some(result)
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
        assert_eq!(result, Some(400));
    }
}
