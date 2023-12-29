use std::{collections::{HashMap, VecDeque, HashSet}, str::FromStr};

use itertools::Itertools;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Tile {
    GardenPlot,
    Rock
}

enum Direction {
    Up,
    Right,
    Down,
    Left
}


#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Coord {
    pub x: usize,
    pub y: usize
}


pub struct Garden {
    grid: Vec<Vec<Tile>>,
    pub rows: usize,
    pub cols: usize
}

impl Garden {
    pub fn take_steps(&self, starting_position: Coord, n: usize,) -> u32 {
        let even = n % 2 == 0;
        let rows = &self.grid.len() - 1;
        let cols = &self.grid[0].len() - 1;

        let mut result: HashSet<Coord> = HashSet::new();
        let mut seen_points: HashSet<Coord> = HashSet::new();
        let mut q = VecDeque::new();
        q.push_front((starting_position, 0));

        while let Some((coord, steps)) = q.pop_front() {
            if steps == n {
                result.insert(coord);
                continue
            }

            let new_steps = steps + 1;

            for dir in vec![Direction::Up, Direction::Right, Direction::Down, Direction::Left] {
                if let Some(v) = &self.validate_step(&coord, dir, rows, cols) {
                    if seen_points.contains(v) {
                        continue;
                    }
                    seen_points.insert(*v);
                    q.push_back((*v, new_steps));

                    match (new_steps % 2 == 0, even) {
                        (true, true) | (false, false) => result.insert(*v),
                        _ => continue
                    };
                }
            }
        }

        result.len() as u32
    }

    fn validate_step(&self, coord: &Coord, dir: Direction, rows: usize, cols: usize) -> Option<Coord> {
        match dir {
            Direction::Up if coord.y > 0 && self.grid[coord.y - 1][coord.x] == Tile::GardenPlot => Some(Coord{ x: coord.x, y: coord.y - 1 }),
            Direction::Right if coord.x < cols && self.grid[coord.y][coord.x + 1] == Tile::GardenPlot => Some(Coord{ x: coord.x + 1, y: coord.y }),
            Direction::Down if coord.y < rows && self.grid[coord.y + 1][coord.x] == Tile::GardenPlot => Some(Coord{ x: coord.x, y: coord.y + 1 }),
            Direction::Left if coord.x > 0 && self.grid[coord.y][coord.x - 1] == Tile::GardenPlot => Some(Coord{ x: coord.x - 1, y: coord.y }),
            _ => None
        }
    }

    pub fn get_starting_position(&self) -> Coord {
        Coord { x: (self.cols as f32 / 2.0).floor() as usize, y: (self.rows as f32 / 2.0).floor() as usize }
    }
}

impl FromStr for Garden {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s
            .lines().map(|line| line.chars().map(|c| {
                match c {
                    '.' => Tile::GardenPlot,
                    '#' => Tile::Rock,
                    'S' => Tile::GardenPlot,
                    _ => panic!("oi")
                }
            }).collect_vec()).collect_vec();

        let rows = grid.clone().len();
        let cols = grid[0].clone().len();

        Ok(Self { grid: grid, rows: rows, cols: cols })
    }
}
