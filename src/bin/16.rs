use std::str::FromStr;

use advent_of_code::repository::beam::{Floor, Beam, Point, Direction};

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<usize> {
    let floor = Floor::from_str(input).expect("Invald floor input");
    Some(floor.energized_tiles(Beam{ position: Point((0, 0)), direction: Direction::Right}))
}

pub fn part_two(input: &str) -> Option<usize> {
    let floor = Floor::from_str(input).expect("Invalid floor input");
    Some(floor.max_energized_tiles())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
