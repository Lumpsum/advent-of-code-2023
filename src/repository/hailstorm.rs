use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug)]
pub struct Coord {
    pub x: i128,
    pub y: i128,
    pub z: i128
}

#[derive(Debug)]
pub struct Velocity {
    pub x: i128,
    pub y: i128,
    pub z: i128
}

#[derive(Debug)]
pub struct HailStone {
    pub position: Coord,
    pub velocity: Velocity,
    a: i128,
    b: i128,
    c: i128
}

impl FromStr for HailStone {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" @ ");
        let position = split.nth(0).unwrap().split(", ").map(|s| s.trim().parse().unwrap()).collect_vec();
        let velocity = split.nth(0).unwrap().split(", ").map(|s| s.trim().parse().unwrap()).collect_vec();
        Ok(
            Self { 
                position: Coord { x: position[0], y: position[1], z: position[2] },
                velocity: Velocity { x: velocity[0], y: velocity[1], z: velocity[2] },
                a: velocity[1],
                b: -velocity[0],
                c: velocity[1] * position[0] - velocity[0] * position[1]
            }
        )
    }
}


pub fn hailstones_collide(hailstone: &HailStone, hailstone_2: &HailStone) -> Option<Coord> {
    let denominator = hailstone.a * hailstone_2.b - hailstone.b * hailstone_2.a;

    if denominator == 0 {
        return None
    }

    let x = (hailstone.c * hailstone_2.b - hailstone.b * hailstone_2.c) / denominator;
    let y = (hailstone.a * hailstone_2.c - hailstone.c * hailstone_2.a) / denominator;
    Some(Coord { x: x, y: y, z: 0 })
}
