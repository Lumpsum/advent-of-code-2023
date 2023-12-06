advent_of_code::solution!(4);
use std::collections::HashMap;
use std::str::FromStr;

use advent_of_code::repository::scratchcard;


pub fn part_one(input: &str) -> Option<u32> {
    let mut result: u32 = 0;
    let base: u32 = 2;

    for line in input.lines() {
        let scratchcard = scratchcard::Scratchcard::from_str(line).unwrap();
        match scratchcard.find_matching_numbers().len() {
            0 => (),
            v => result += base.pow((v - 1).try_into().unwrap())
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result: usize = 0;
    let mut copies: HashMap<usize, usize> = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        let scratchcard = scratchcard::Scratchcard::from_str(line).unwrap();
        let current_copies: usize = match copies.get(&i) {
            Some(v) => *v,
            None => 0
        };

        match scratchcard.find_matching_numbers().len() {
            0 => (),
            v => {
                for j in 0..v {
                    *copies.entry(i + (j + 1)).or_insert(0) += 1 + current_copies;
                }
            }
        }

        result += 1 + current_copies
    }
    Some(result.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
