use std::str::FromStr;

use itertools::Itertools;


#[derive(Clone, Copy)]
struct Node {
    x: i64,
    y: i64
}


#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}


struct DigStep {
    direction: Direction,
    steps: i64,
    rgb: String
}


pub struct DigPlan(Vec<DigStep>);

impl DigPlan {
    pub fn calculate_area(&self) -> u32 {
        let mut current_node = Node{ x: 0, y: 0 };
        let mut nodes = vec![current_node];

        for step in &self.0 {
            current_node = Self::calculate_new_node(current_node, step);
            nodes.push(current_node);
        }

        (nodes.iter().tuple_windows().map(|(prev, next)| {
            prev.x * next.y - next.x * prev.y
        }).sum::<i64>() as f32 * 0.5) as u32
    }

    pub fn calculate_perimeter(&self) -> u32 {
        self.0.iter().map(|s| s.steps).sum::<i64>() as u32
    }

    pub fn rgb_steps(&self) -> i64 {
        let mut area: i64 = 0;
        let mut perimeter: i64 = 0;
        let mut current_node = Node{ x: 0, y: 0 };

        for step in &self.0 {
            let rgb_chars = step.rgb.chars();
            let step = rgb_chars.clone().skip(2).take(5).collect::<String>();
            let steps = i64::from_str_radix(&step, 16).unwrap();
            perimeter += steps;
            
            let direction = match rgb_chars.skip(7).take(1).next().unwrap() {
                '0' => Direction::Right,
                '1' => Direction::Down,
                '2' => Direction::Left,
                '3' => Direction::Up,
                _ => panic!("wtf")
            };

            let new_node = Self::calculate_new_node(current_node, &DigStep { direction: direction, steps: steps, rgb: String::new() });
            area += current_node.x * new_node.y - new_node.x * current_node.y;
            current_node = new_node;
        }

        (0.5 * area as f64) as i64 + perimeter / 2 + 1
    }

    fn calculate_new_node(mut node: Node, step: &DigStep) -> Node {
        match step.direction {
            Direction::Up => node.y -= step.steps,
            Direction::Right => node.x += step.steps,
            Direction::Down => node.y += step.steps,
            Direction::Left => node.x -= step.steps,
        }
        node
    }
}

impl FromStr for DigPlan {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(
            Self(
                s.lines()
                .map(|line| {
                    let splits = line.split(" ").collect_vec();
                    let direction = match splits[0] {
                        "U" => Direction::Up,
                        "R" => Direction::Right,
                        "D" => Direction::Down,
                        "L" => Direction::Left,
                        _ => panic!("wtf")
                    };

                    DigStep { direction: direction, steps: splits[1].parse().unwrap(), rgb: String::from(splits[2]) }
                }).collect_vec()
            )
        )
    }
}
