use std::{str::FromStr, collections::{VecDeque, HashSet, HashMap}, cmp::min};

#[derive(Debug)]
pub struct PipeNetwork {
    pub grid: Vec<Vec<char>>,
    pub starting_point: (i32, i32)
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
    pub result: u32,
}

impl Graph {
    fn get_options(c: &char) -> Vec<(i32, i32, Direction)> {
        match c {
            '|' => return vec![(0, -1, Direction::UP), (0, 1, Direction::DOWN)],
            '-' => return vec![(-1, 0, Direction::LEFT), (1, 0, Direction::RIGHT)],
            'L' => return vec![(0, -1, Direction::UP), (1, 0, Direction::RIGHT)],
            'J' => return vec![(0, -1, Direction::UP), (-1, 0, Direction::LEFT)],
            '7' => return vec![(0, 1, Direction::DOWN), (-1, 0, Direction::LEFT)],
            'F' => return vec![(0, 1, Direction::DOWN), (1, 0, Direction::RIGHT)],
            'S' => return vec![(0, -1, Direction::UP), (0, 1, Direction::DOWN), (-1, 0, Direction::LEFT), (1, 0, Direction::RIGHT)],
            _ => return vec![]
        }
    }

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

    pub fn point_in_polygon(&self, point: (i32, i32)) -> bool {
        if self.nodes.contains(&point) |
            (self.nodes.iter().filter(|&n| n.0 < point.0 && n.1 == point.1).count() == 0) |
            (self.nodes.iter().filter(|&n| n.1 < point.1 && n.0 == point.0).count() == 0) |
            (self.nodes.iter().filter(|&n| n.1 > point.1 && n.0 == point.0).count() == 0) {
            return false
        }

        let mut crossed_edges = 0;
        let mut up = 0;
        let mut down = 0;

        for (node, edges) in self.edges.iter().filter(|&v| v.0.1 == point.1 && v.0.0 > point.0).collect::<Vec<(&(i32, i32), &HashSet<(i32, i32)>)>>() {
            let mut total = 0;

            match edges.get(&(node.0, node.1 - 1)) {
                Some(_) => {
                    up += 1;
                    total += 1
                },
                None => ()
            }

            match edges.get(&(node.0, node.1 + 1)) {
                Some(_) => {
                    down += 1;
                    total += 1
                },
                None => ()
            }

            if total == 2 {
                crossed_edges += 1;
                up -= 1;
                down -= 1;
            }
        };

        (crossed_edges + min(up, down)) % 2 != 0
    }
}

impl From<&PipeNetwork> for Graph {
    fn from(value: &PipeNetwork) -> Self {
        let mut queue: VecDeque<Vec<(i32, i32)>> = VecDeque::new();
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut edges: HashMap<(i32, i32), HashSet<(i32, i32)>> = HashMap::new();
        queue.push_back(vec![value.starting_point]);
        visited.insert(value.starting_point);

        let mut result = 0;

        while let Some(v) = queue.pop_front() {
            let mut new_nodes: Vec<(i32, i32)> = Vec::new();

            for node in v {
                for option in Graph::get_options(value.grid.get(node.1 as usize).unwrap().get(node.0 as usize).unwrap()) {
                    let i = node.0 + option.0;
                    let j = node.1 + option.1;

                    match value.grid.get(j as usize) {
                        Some(vec) => {
                            match vec.get(i as usize) {
                                Some(c) => {
                                    match Self::valid_node(*c, option.2) {
                                        true => {
                                            edges.entry(node).or_insert(HashSet::new()).insert((i, j));
                                            edges.entry((i, j)).or_insert(HashSet::new()).insert(node);

                                            match visited.insert((i, j)) {
                                                true => {
                                                    new_nodes.push((i, j));

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
