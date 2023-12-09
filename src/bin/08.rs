advent_of_code::solution!(8);
use advent_of_code::repository::map::Maps;


pub fn part_one(input: &str) -> Option<u32> {
    let maps = Maps::from(input);
    Some(maps.find_single_route())
}

pub fn part_two(input: &str) -> Option<u64> {
    let maps = Maps::from(input);
    Some(maps.find_simultaneous_route())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
