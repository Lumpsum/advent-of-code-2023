use core::panic;
use std::{str::FromStr, collections::{VecDeque, HashSet}};

use itertools::Itertools;

#[derive(Clone, Copy)]
enum Tile {
    Empty,
    HorizontalSplitter,
    VerticalSplitter,
    MirrorForward,
    MirrorBack,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct Point(pub (usize, usize));

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct Beam {
    pub position: Point,
    pub direction: Direction
}

impl Beam {
    fn forward(mut self, rows: usize, cols: usize) -> Option<Self> {
        match self.direction {
            Direction::Up if self.position.0.1 > 0 => self.position.0.1 -= 1,
            Direction::Down if self.position.0.1 < rows - 1 => self.position.0.1 += 1,
            Direction::Left if self.position.0.0 > 0 => self.position.0.0 -= 1,
            Direction::Right if self.position.0.0 < cols - 1 => self.position.0.0 += 1,
            _ => return None
        }
        Some(self)
    }
}

pub struct Floor(Vec<Vec<Tile>>);

impl Floor {
    pub fn max_energized_tiles(&self) -> usize {
        let rows = self.0.len();
        let cols = self.0[0].len();

        let left = (0..rows).map(|row| Beam {
            position: Point((cols - 1, row)),
            direction: Direction::Left,
        });
        let right = (0..rows).map(|row| Beam {
            position: Point((0, row)),
            direction: Direction::Right
        });
        let up = (0..cols).map(|col| Beam{
            position: Point((col, rows - 1)),
            direction: Direction::Up,
        });
        let down = (0..cols).map(|col| Beam{
            position: Point((col, 0)),
            direction: Direction::Down
        });

        left.chain(right).chain(down).chain(up)
            .map(|start| self.energized_tiles(start))
            .max()
            .unwrap()
    }

    pub fn energized_tiles(&self, beam: Beam) -> usize {
        let rows = self.0.len();
        let cols = self.0[0].len();
        
        let mut queue = VecDeque::new();
        let mut energized = HashSet::new();
        let mut seen = HashSet::new();
        queue.push_back(beam);
        
        while let Some(mut beam) = queue.pop_front() {
            if seen.contains(&beam) {
                continue;
            }
            energized.insert(beam.position);
            seen.insert(beam);

            let directions = match (self.0[beam.position.0.1][beam.position.0.0], beam.direction) {
                (Tile::Empty, _)
                | (Tile::HorizontalSplitter, Direction::Left)
                | (Tile::HorizontalSplitter, Direction::Right)
                | (Tile::VerticalSplitter, Direction::Up)
                | (Tile::VerticalSplitter, Direction::Down) => vec![beam.direction],
                (Tile::HorizontalSplitter, _) => vec![Direction::Left, Direction::Right],
                (Tile::VerticalSplitter, _) => vec![Direction::Up, Direction::Down],
                (Tile::MirrorForward, Direction::Up) | (Tile::MirrorBack, Direction::Down) => vec![Direction::Right],
                (Tile::MirrorForward, Direction::Down) | (Tile::MirrorBack, Direction::Up) => vec![Direction::Left],
                (Tile::MirrorForward, Direction::Left) | (Tile::MirrorBack, Direction::Right) => vec![Direction::Down],
                (Tile::MirrorForward, Direction::Right) | (Tile::MirrorBack, Direction::Left) => vec![Direction::Up],
            };

            for direction in directions {
                beam.direction = direction;
                if let Some(beam) = beam.forward(rows, cols) {
                    queue.push_back(beam);
                }
            }
        }

        energized.len()
    }
}

impl FromStr for Floor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(
            Self(
                s.lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '\\' => Tile::MirrorBack,
                            '/' => Tile::MirrorForward,
                            '-' => Tile::HorizontalSplitter,
                            '|' => Tile::VerticalSplitter,
                            '.' => Tile::Empty,
                            _ => panic!("wtf")
                        })
                        .collect_vec()
                })
                .collect_vec()
            )
        )
    }
}
