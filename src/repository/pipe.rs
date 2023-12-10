use std::{str::FromStr, collections::{VecDeque, HashSet, HashMap}};

#[derive(Debug)]
pub struct PipeNetwork {
    grid: Vec<Vec<char>>,
    starting_point: (i32, i32)
}

impl FromStr for PipeNetwork {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid: Vec<Vec<char>> = Vec::new();
        let mut staring_point: (i32, i32) = (0, 0);

        for (i, line) in s.lines().enumerate() {
            grid.push(line.chars().enumerate().map(|(j, char)| {
                if char == 'S' {
                    staring_point = (j.try_into().unwrap(), i.try_into().unwrap())
                }
                char
            }).collect());
        }

        Ok(Self{
            grid: grid,
            starting_point: staring_point,
        })
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

pub struct Graph {
    nodes: HashSet<(i32, i32)>,
    edges: HashMap<(i32, i32), HashSet<(i32, i32)>>,
    pub result: u32
}

impl Graph {
    fn valid_node(symbol: char, direction: Direction) -> bool {
        match direction {
            Direction::UP => {
                match symbol {
                    '|' => return true,
                    '7' => return true,
                    'F' => return true,
                    _ => return false
                }
            },
            Direction::DOWN => {
                match symbol {
                    '|' => return true,
                    'L' => return true,
                    'J' => return true,
                    _ => return false
                }
            },
            Direction::LEFT => {
                match symbol {
                    '-' => return true,
                    'L' => return true,
                    'F' => return true,
                    _ => return false
                }
            },
            Direction::RIGHT => {
                match symbol {
                    '-' => return true,
                    'J' => return true,
                    '7' => return true,
                    _ => return false
                }
            }
        }
    }
}

impl From<PipeNetwork> for Graph {
    fn from(value: PipeNetwork) -> Self {
        let options: Vec<(i32, i32, Direction)> = vec![
            (0, -1, Direction::UP), (0, 1, Direction::DOWN),
            (-1, 0, Direction::LEFT), (1, 0, Direction::RIGHT)
        ];
        let mut queue: VecDeque<Vec<(i32, i32)>> = VecDeque::new();
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut edges: HashMap<(i32, i32), HashSet<(i32, i32)>> = HashMap::new();
        queue.push_back(vec![value.starting_point]);
        visited.insert(value.starting_point);

        let mut result = 0;

        while let Some(v) = queue.pop_front() {
            let mut new_nodes: Vec<(i32, i32)> = Vec::new();

            for node in v {
                for option in options.clone() {
                    let i = node.0 + option.0;
                    let j = node.1 + option.1;

                    match value.grid.get(j as usize) {
                        Some(vec) => {
                            match vec.get(i as usize) {
                                Some(c) => {
                                    match Self::valid_node(*c, option.2) {
                                        true => {
                                            edges.entry(node).or_insert(HashSet::new()).insert((i, j));
                                            match visited.insert((i, j)) {
                                                true => {
                                                    new_nodes.push((i, j))
                                                },
                                                false => ()
                                            }
                                        },
                                        false => ()
                                    }
                                },
                                None => ()
                            }
                        },
                        None => ()
                    }
                }
            }

            if !new_nodes.is_empty() {
                result += 1;
                queue.push_back(new_nodes);
            }
        }

        Self{
            nodes: visited,
            edges: edges,
            result: result
        }
    }
}
