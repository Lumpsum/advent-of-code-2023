advent_of_code::solution!(10);
use std::str::FromStr;

use advent_of_code::repository::pipe::{PipeNetwork, Graph};


pub fn part_one(input: &str) -> Option<u32> {
    let pipe_network = PipeNetwork::from_str(input).unwrap();
    let graph = Graph::from(&pipe_network);
    Some(graph.result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let pipe_network = PipeNetwork::from_str(input).unwrap();
    let graph = Graph::from(&pipe_network);

    let mut result: u32 = 0;
    for j in pipe_network.grid.iter().enumerate() {
        for i in j.1.iter().enumerate() {
            if graph.point_in_polygon((i.0 as i32, j.0 as i32)) {
                println!("{:?}", (i.0, j.0));
                result += 1
            }
        }
    }

    // println!("{:?}", graph.horizontal_edges);
    // println!("{:?}", graph.point_in_polygon((5, 7)));

    // 518 too high
    Some(result)
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
        assert_eq!(result, Some(10));
    }
}
