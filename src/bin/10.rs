advent_of_code::solution!(10);
use std::str::FromStr;

use advent_of_code::repository::pipe::{PipeNetwork, Graph};


pub fn part_one(input: &str) -> Option<u32> {
    let pipe_network = PipeNetwork::from_str(input).unwrap();
    let graph = Graph::from(pipe_network);
    // 13803 too high
    Some(graph.result)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
