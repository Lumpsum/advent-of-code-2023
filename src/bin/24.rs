use std::str::FromStr;

use advent_of_code::repository::hailstorm::{HailStone, hailstones_collide};
use itertools::Itertools;

advent_of_code::solution!(24);

pub fn part_one(input: &str) -> Option<u32> {
    let hail_stones = input.lines().map(|line| HailStone::from_str(line).unwrap()).collect_vec();

    let lower_limit = 200000000000000;
    let upper_limit = 400000000000000;
    let mut result = 0;

    for hailstone_ in hail_stones.iter().combinations(2) {
        if let Some(coord) = hailstones_collide(hailstone_[0], hailstone_[1]) {
            if (hailstone_.iter().all(|&hailstone| (coord.x - hailstone.position.x) * hailstone.velocity.x >= 0 && (coord.y - hailstone.position.y) * hailstone.velocity.y >= 0))
               && (coord.x >= lower_limit && coord.x <= upper_limit && coord.y >= lower_limit && coord.y <= upper_limit) {
                result += 1;
            }
        }

    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let hail_stones = input.lines().map(|line| HailStone::from_str(line).unwrap()).collect_vec();



    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
