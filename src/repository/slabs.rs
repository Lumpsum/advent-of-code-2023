use std::{str::FromStr, cmp::{min, max}, collections::{HashMap, HashSet}};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Coord {
    x: isize,
    y: isize,
    pub z: isize
}


#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Brick {
    pub start: Coord,
    pub end: Coord
}

impl Brick {
    pub fn overlaps(&self, brick: &Brick) -> bool {
        max(self.start.x, brick.start.x) <= min(self.end.x, brick.end.x) && max(self.start.y, brick.start.y) <= min(self.end.y, brick.end.y)
    }
}

impl FromStr for Brick {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("~");
        let start = split.nth(0).unwrap().split(",").map(|c| c.parse::<isize>().unwrap()).collect_vec();
        let end = split.nth(0).unwrap().split(",").map(|c| c.parse::<isize>().unwrap()).collect_vec();
        Ok(Self { start: Coord { x: start[0], y: start[1], z: start[2] }, end: Coord { x: end[0], y: end[1], z: end[2] } })
    }
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        min(self.start.z, self.end.z).cmp(min(&other.start.z, &other.end.z))
    }
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}


pub fn adjust_bricks(mut bricks: Vec<Brick>) -> Vec<Brick> {
    let mut new_bricks: Vec<Brick> = Vec::new();

    for brick in bricks.iter_mut() {
        let mut max_z = 1;
        for j in new_bricks.clone() {
            if brick.overlaps(&j) {
                max_z = max(max_z, j.end.z + 1);
            }
        }

        brick.end.z -= brick.start.z - max_z;
        brick.start.z = max_z;
        new_bricks.push(*brick);
    }
    
    new_bricks
}


pub fn get_support_bricks(bricks: &Vec<Brick>) -> (HashMap<usize, HashSet<usize>>, HashMap<usize, HashSet<usize>>) {
    let mut supports = HashMap::new();
    let mut is_supported = HashMap::new();

    for (j, upper_brick) in bricks.iter().enumerate() {
        supports.entry(j).or_insert(HashSet::new());
        is_supported.entry(j).or_insert(HashSet::new());
        for (i, lower_brick) in bricks[..j].iter().enumerate() {
            if lower_brick.overlaps(upper_brick) && lower_brick.end.z + 1 == upper_brick.start.z {
                supports.get_mut(&i).unwrap().insert(j);
                is_supported.get_mut(&j).unwrap().insert(i);
            }
        }
    }

    (supports, is_supported)
}
