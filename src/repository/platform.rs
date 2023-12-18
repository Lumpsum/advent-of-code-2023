use std::{str::FromStr, collections::HashMap};

use itertools::Itertools;


pub enum Direction {
    NORTH,
    WEST,
    SOUTH,
    EAST
}


#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RoundRockSet(pub Vec<(isize, isize)>);
impl RoundRockSet {
    fn new() -> Self {
        Self(Vec::new())
    }
}

struct CubeRockSet(Vec<(isize, isize)>);


pub struct Platform {
    pub round_rocks: RoundRockSet,
    cube_rocks: CubeRockSet,
    height: isize,
    width: isize
}

impl Platform {
    pub fn total_load(&self, direction: Direction) -> u32 {
        match direction {
            Direction::NORTH => {
                let weight = (&self.height + 1) as isize;
                return self.round_rocks.0.iter().map(|r| weight - r.1).sum::<isize>().try_into().unwrap()
            },
            Direction::WEST => todo!(),
            Direction::SOUTH => todo!(),
            Direction::EAST => todo!(),
        }
    }

    pub fn cycle_n_times(&mut self, n: i64) {
        let mut memory: HashMap<RoundRockSet, i64> = HashMap::new();

        for i in 0..n {
            match memory.get(&self.round_rocks) {
                Some(v) => {
                    let ending_iteration = (n - 1 - v) % (i - v);
                    for _ in 0..=ending_iteration {
                        self.cycle()
                    }
                    break
                },
                None => {
                    let old_rocks = self.round_rocks.clone();
                    self.cycle();
                    memory.insert(old_rocks, i);
                }
            }
        }
    }

    fn cycle(&mut self) {
        self.tilt(Direction::NORTH);
        self.tilt(Direction::WEST);
        self.tilt(Direction::SOUTH);
        self.tilt(Direction::EAST);
    }

    pub fn tilt(&mut self, direction: Direction) {
        match direction {
            Direction::NORTH => {
                let mut new_rocks = RoundRockSet::new();
                let sorted_rocks = self.round_rocks.0.iter().sorted_by_key(|&x|x.1).collect_vec();
                sorted_rocks.iter().map(|&r| {
                    let mut new_r = r.clone();
                    loop {
                        new_r.1 -= 1;
                        if new_rocks.0.contains(&new_r) || self.cube_rocks.0.contains(&new_r) || new_r.1 < 0 {
                            new_r.1 += 1;
                            break
                        }
                    }
                    new_rocks.0.push(new_r);
                }).collect_vec();
                self.round_rocks = new_rocks;
            },
            Direction::EAST => {
                let mut new_rocks = RoundRockSet::new();
                let sorted_rocks = self.round_rocks.0.iter().sorted_by_key(|&x|-x.0).collect_vec();
                sorted_rocks.iter().map(|&r| {
                    let mut new_r = r.clone();
                    loop {
                        new_r.0 += 1;
                        if new_rocks.0.contains(&new_r) || self.cube_rocks.0.contains(&new_r) || new_r.0 > self.width {
                            new_r.0 -= 1;
                            break
                        }
                    }
                    new_rocks.0.push(new_r);
                }).collect_vec();
                self.round_rocks = new_rocks;
            },
            Direction::SOUTH => {
                let mut new_rocks = RoundRockSet::new();
                let sorted_rocks = self.round_rocks.0.iter().sorted_by_key(|&x|-x.1).collect_vec();
                sorted_rocks.iter().map(|&r| {
                    let mut new_r = r.clone();
                    loop {
                        new_r.1 += 1;
                        if new_rocks.0.contains(&new_r) || self.cube_rocks.0.contains(&new_r) || new_r.1 > self.height {
                            new_r.1 -= 1;
                            break
                        }
                    }
                    new_rocks.0.push(new_r);
                }).collect_vec();
                self.round_rocks = new_rocks;
            }
            Direction::WEST => {
                let mut new_rocks = RoundRockSet::new();
                let sorted_rocks = self.round_rocks.0.iter().sorted_by_key(|&x|x.0).collect_vec();
                sorted_rocks.iter().map(|&r| {
                    let mut new_r = r.clone();
                    loop {
                        new_r.0 -= 1;
                        if new_rocks.0.contains(&new_r) || self.cube_rocks.0.contains(&new_r) || new_r.0 < 0 {
                            new_r.0 += 1;
                            break
                        }
                    }
                    new_rocks.0.push(new_r);
                }).collect_vec();
                self.round_rocks = new_rocks;
            }
        };
    }
}

impl FromStr for Platform {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut round_rocks = Vec::new();
        let mut cube_rocks = Vec::new();
        let mut height = 0;
        let mut width = 0;

        for (j, line) in s.lines().enumerate() {
            for (i, char) in line.chars().enumerate() {
                match char {
                    'O' => round_rocks.push((i as isize, j as isize)),
                    '#' => cube_rocks.push((i as isize, j as isize)),
                    _ => ()
                }
                width = i;
            }
            height = j
        }

        Ok(Self { round_rocks: RoundRockSet(round_rocks), cube_rocks: CubeRockSet(cube_rocks), height: height as isize, width: width as isize })
    }
}
