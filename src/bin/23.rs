use advent_of_code::repository::walk::{Graph, Grid, Node};

advent_of_code::solution!(23);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::from((input, true));
    let graph = Graph::from(&grid);
    Some(graph.longest_path(Node{ x: 1, y: 0 }, Node{ x: grid.cols - 2, y: grid.rows - 1}))
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::from((input, false));
    let graph = Graph::from(&grid);
    Some(graph.longest_path(Node{ x: 1, y: 0 }, Node{ x: grid.cols - 2, y: grid.rows - 1}))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154));
    }
}
