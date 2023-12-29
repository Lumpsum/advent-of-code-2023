use std::{str::FromStr, collections::{VecDeque, HashSet}};

use advent_of_code::repository::slabs::{Brick, adjust_bricks, get_support_bricks};
use itertools::Itertools;

advent_of_code::solution!(22);

pub fn part_one(input: &str) -> Option<usize> {
    let mut bricks = input.lines().map(|l| Brick::from_str(l).unwrap()).collect_vec();
    bricks.sort();

    bricks = adjust_bricks(bricks);
    let (supports, is_supported) = get_support_bricks(&bricks);

    Some(supports.into_iter().filter(|brick| brick.1.iter().all(|i| is_supported.get(i).unwrap().len() >= 2)).collect_vec().len())
}

pub fn part_two(input: &str) -> Option<usize> { 
    let mut bricks = input.lines().map(|l| Brick::from_str(l).unwrap()).collect_vec();
    bricks.sort();

    bricks = adjust_bricks(bricks);
    let (supports, is_supported) = get_support_bricks(&bricks);

    let mut total = 0;
    for (i, supported) in &supports {
        let mut q = VecDeque::new();
        let mut falling = HashSet::new();
        falling.insert(i);
        for j in supported {
            if is_supported[&j].len() == 1 {
                q.push_front(j);
                falling.insert(j);
            }
        }

        while let Some(v) = q.pop_front() {
            for j in supports.get(&v).unwrap() {
                if is_supported.get(j).unwrap().iter().all(|b| falling.contains(b)) {
                    falling.insert(j);
                    q.push_back(j);
                }
            }
        }

        total += falling.len() - 1;
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }
}
