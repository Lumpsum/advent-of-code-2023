advent_of_code::solution!(3);
use std::str::FromStr;

use advent_of_code::repository::engine;

pub fn part_one(input: &str) -> Option<u32> {
    let engine_schematic = engine::EngineSchematic::from_str(input).unwrap();
    let mut valid_numbers = engine_schematic.find_valid_numbers();
    valid_numbers.dedup_by(|a, b| a.id == b.id);

    Some(valid_numbers.iter().map(|s| s.value).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let engine_schematic = engine::EngineSchematic::from_str(input).unwrap();
    Some(engine_schematic.find_gears().iter().map(|s| s).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
