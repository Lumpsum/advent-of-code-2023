advent_of_code::solution!(7);

use advent_of_code::repository::camel::Hand;

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands: Vec<Hand> = input.lines().map(|s| Hand::from((s, false))).collect();
    hands.sort();
    Some(
        hands.iter().enumerate()
        .map(|(i, hand): (usize, &Hand<'_>)| -> u32 { hand.strength * (u32::try_from(i).unwrap() + 1) })
        .sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands: Vec<Hand> = input.lines().map(|s| Hand::from((s, true))).collect();
    hands.sort();
    Some(
        hands.iter().enumerate()
        .map(|(i, hand): (usize, &Hand<'_>)| -> u32 { hand.strength * (u32::try_from(i).unwrap() + 1) })
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
