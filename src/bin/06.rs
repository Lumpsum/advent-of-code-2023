advent_of_code::solution!(6);

use advent_of_code::repository::race;


pub fn part_one(input: &str) -> Option<u32> {
    let mut result: u32 = 1;
    let mut lines = input.lines();
    let time = race::extract_numbers(lines.nth(0).unwrap());
    let distance = race::extract_numbers(lines.nth(0).unwrap());

    let races: Vec<race::Race> = race::create_races(time, distance);
    for race in races {
        result *= race.calculate_ways_of_beating();
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let time = race::extract_text(lines.nth(0).unwrap());
    let distance = race::extract_text(lines.nth(0).unwrap());

    let race = race::Race{
        time: time.iter().fold("".to_string(), |a: String, b: &&str| a + b).parse().unwrap(),
        distance: distance.iter().fold("".to_string(), |a: String, b: &&str| a + b).parse().unwrap()
    };

    Some(race.calculate_ways_of_beating())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
