advent_of_code::solution!(15);
use std::collections::HashMap;

use advent_of_code::repository::hash::{Sequence, Box, Boxes};

pub fn part_one(input: &str) -> Option<usize> {
    Some(input.split(",").map(|s| Sequence(s).solve()).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut boxes = Boxes(vec![Box(Vec::new(), HashMap::new()); 255]);

    for s in input.split(",") {
        boxes = Sequence(s).hashmap(boxes);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
