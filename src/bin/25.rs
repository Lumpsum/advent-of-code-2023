use advent_of_code::repository::snowverload::from_input;
use rustworkx_core::connectivity::stoer_wagner_min_cut;
use rustworkx_core::Result;

advent_of_code::solution!(25);

pub fn part_one(input: &str) -> Option<usize> {
    let graph = from_input(input);
    let min_cut_res: Result<Option<(usize, Vec<_>)>> = stoer_wagner_min_cut(&graph, |_| Ok(1));
    let (_, partition) = min_cut_res.unwrap().unwrap();

    Some(partition.len() * (graph.node_count() - partition.len()))
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
        assert_eq!(result, Some(54));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
