use std::{cmp::Ordering, str::FromStr, collections::{BinaryHeap, HashSet}};

use itertools::Itertools;

#[derive(Copy, Clone, Eq, PartialEq, Debug, PartialOrd, Ord, Hash)]
enum Move {
    Up,
    Right,
    Down,
    Left
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct Coord {
    x: usize,
    y: usize
}


#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct CityBlock {
    position: Coord,
    cost: u32
}


impl Ord for CityBlock {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| (self.position.x + self.position.y).cmp(&(other.position.x + other.position.y)))
    }
}

impl PartialOrd for CityBlock {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
pub struct Map(Vec<Vec<u32>>);
impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(
            Map(
                s.lines()
                .enumerate()
                .map(|v| v.1.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect_vec()
                )
                .collect_vec()
            )
        )
    }
}


pub fn shortest_path(map: Map, min_steps: usize, max_steps: usize) -> Option<u32> {
    let rows = map.0.len() - 1;
    let cols = map.0[0].len() - 1;
    let goal = Coord{ x: cols, y: rows };

    let mut lowest: HashSet<(Coord, Move, u32)> = HashSet::new();
    let mut binary_heap: BinaryHeap<(CityBlock, Move, u32)> = BinaryHeap::new();

    binary_heap.push((CityBlock { position: Coord { x: 0, y: 0 }, cost: 0}, Move::Up, 0));
    binary_heap.push((CityBlock { position: Coord { x: 0, y: 0 }, cost: 0}, Move::Left, 0));

    while let Some(city_block) = binary_heap.pop() {
        if city_block.0.position == goal { return Some(city_block.0.cost) }

        let new_directions = match city_block.1 {
            Move::Up => vec![Move::Left, Move::Right],
            Move::Right => vec![Move::Up, Move::Down],
            Move::Down => vec![Move::Left, Move::Right],
            Move::Left => vec![Move::Up, Move::Down],
        };

        for direction in new_directions {
            let mut new_city_block = city_block.0.clone();

            for _ in 1..min_steps {
                if let Some(new_block) = get_new_blocks(direction, new_city_block, rows, cols, &map) {
                    new_city_block = new_block;
                }
            }

            for i in min_steps..=max_steps {
                if let Some(new_block) = get_new_blocks(direction, new_city_block, rows, cols, &map) {
                    new_city_block = new_block;
                    let moves = i as u32;

                    if !lowest.contains(&(new_city_block.position, direction, moves)) {
                        lowest.insert((new_city_block.position, direction, moves));
                        binary_heap.push((new_city_block, direction, moves));
                    }
                }
            }
        }
    }

    None
}


fn get_new_blocks(m: Move, mut city_block: CityBlock, rows: usize, cols: usize, map: &Map) -> Option<CityBlock> {
    match m {
        Move::Up if city_block.position.y > 0 => city_block.position.y -= 1,
        Move::Right if city_block.position.x < cols => city_block.position.x += 1,
        Move::Down if city_block.position.y < rows => city_block.position.y += 1,
        Move::Left if city_block.position.x > 0 => city_block.position.x -= 1,
        _ => return None
    }

    city_block.cost += map.0[city_block.position.y][city_block.position.x];
    Some(city_block)
}
