use advent_of_code::repository::extrapolate::History;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i32> {
    Some(
        input.lines()
        .map(|s| 
            History(s.split(" ")
            .map(|c| c.parse().unwrap()).collect())
            .find_next_value()
        ).sum()
    )
}

pub fn part_two(input: &str) -> Option<i32> {
    Some(
        input.lines()
        .map(|s| {
            let mut h: Vec<i32> = s.split(" ").map(|c| c.parse().unwrap()).collect();
            h.reverse();
            History(h).find_next_value()
        }).sum()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
