use std::{str::FromStr, collections::{HashSet, VecDeque, HashMap}};

use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Tile {
    Path,
    Forest,
    SlopeUp,
    SlopeRight,
    SlopeDown,
    SlopeLeft
}

enum Direction {
    Up,
    Right,
    Down,
    Left
}


pub struct Grid{
    pub rows: usize,
    pub cols: usize,
    grid: Vec<Vec<Tile>>
}

impl Grid {
    fn get_edge_contracted_nodes(&self) -> Vec<Node> {
        let mut nodes = Vec::new();

        for j in 0..self.rows {
            for i in 0..self.cols {
                match self.grid[j][i] {
                    Tile::Path => {
                        if vec![Direction::Up, Direction::Right, Direction::Down, Direction::Left].iter().map(|d| self.valid_step(Node { x: i, y: j }, d)).filter(|b| match b {
                            Some(_) => true,
                            _ => false
                        }).collect_vec().len() != 2 {
                            nodes.push(Node{ x: i, y: j })
                        }
                    },
                    _ => continue
                };
            };
        };

        nodes
    }

    fn get_edges_from_edge_contracted_nodes(&self, nodes: &Vec<Node>) -> HashMap<Node, Vec<(Node, usize)>> {
        let mut edges = HashMap::new();
        let mut seen_nodes = HashSet::new();
        seen_nodes.insert(Node{ x: &self.cols - 2, y: &self.rows - 1});

        let mut node_q = VecDeque::new();
        node_q.push_front(Node { x: 1, y: 0});

        while let Some(node) = node_q.pop_front() {
            let mut seen_tiles = HashSet::new();
            seen_nodes.insert(node);
            seen_tiles.insert(node);

            let mut q = VecDeque::new();
            q.push_front((node, 0));

            while let Some(v) = q.pop_front() {
                if v.1 != 0 && nodes.contains(&v.0) {
                    edges.entry(node).or_insert(Vec::new()).push((v.0, v.1 as usize));
                    seen_tiles.insert(v.0);

                    if !seen_nodes.contains(&v.0) {
                        node_q.push_back(v.0);
                        seen_nodes.insert(v.0);
                    }
                    continue;
                }

                let directions = &self.get_valid_directions(&node);
                for direction in directions {
                    match self.valid_step(v.0, direction) {
                        Some(n) => {
                            if seen_tiles.contains(&n) {
                                continue;
                            }

                            seen_tiles.insert(n);
                            q.push_back((n, v.1 + 1))                          
                        },
                        None => continue
                    }
                }
            }
        }

        edges
    }

    fn get_valid_directions(&self, node: &Node) -> Vec<Direction> {
        match &self.grid[node.y][node.x] {
            Tile::Path => vec![Direction::Up, Direction::Right, Direction::Down, Direction::Left],
            Tile::Forest => todo!(),
            Tile::SlopeUp => vec![Direction::Up],
            Tile::SlopeRight => vec![Direction::Right],
            Tile::SlopeDown => vec![Direction::Down],
            Tile::SlopeLeft => vec![Direction::Left],
        }
    }

    fn valid_step(&self, node: Node, direction: &Direction) -> Option<Node> {
        match direction {
            Direction::Up if node.y > 0 && vec![Tile::Path, Tile::SlopeUp].contains(&self.grid[node.y - 1][node.x]) => Some(Node{ x: node.x, y: node.y - 1}),
            Direction::Right if node.x < self.cols - 1 && vec![Tile::Path, Tile::SlopeRight].contains(&self.grid[node.y][node.x + 1]) => Some(Node{ x: node.x + 1, y: node.y }),
            Direction::Down if node.y < self.rows - 1 && vec![Tile::Path, Tile::SlopeDown].contains(&self.grid[node.y + 1][node.x]) => Some(Node{ x: node.x, y: node.y + 1}),
            Direction::Left if node.x > 0 && vec![Tile::Path, Tile::SlopeLeft].contains(&self.grid[node.y][node.x - 1]) => Some(Node{ x: node.x - 1, y: node.y }),
            _ => None
        }
    }
}

impl From<(&str, bool)> for Grid {
    fn from(value: (&str, bool)) -> Self {
        let grid = value.0.lines().map(|line| line.chars().map(|c| match value.1 {
                true => match c {
                    '.' => Tile::Path,
                    '#' => Tile::Forest,
                    '^' => Tile::SlopeUp,
                    '>' => Tile::SlopeRight,
                    'v' => Tile::SlopeDown,
                    '<' => Tile::SlopeLeft,
                    _ => panic!("wtf"),
                }
                false => match c {
                    '#' => Tile::Forest,
                    _ => Tile::Path
                } 
        }).collect_vec()).collect_vec();
        Self{
            rows: grid.clone().len(),
            cols: grid.clone()[0].len(),
            grid: grid
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Node {
    pub x: usize,
    pub y: usize
}

#[derive(Debug)]
pub struct Edge {
    from: Node,
    to: Node,
    cost: usize
}

pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: HashMap<Node, Vec<(Node, usize)>>
}

impl Graph {
    pub fn longest_path(&self, starting_node: Node, end_node: Node) -> u32 {
        let mut q = VecDeque::new();
        q.push_front((vec![starting_node], 0));
        let mut valid_paths = Vec::new();

        while let Some((mut nodes, cost)) = q.pop_front() {
            let last_node = nodes.last().unwrap();
            if last_node == &end_node {
                valid_paths.push(cost);
                continue;
            }

            for new_node in self.edges.get(last_node).unwrap() {
                if !nodes.contains(&new_node.0) {
                    let mut new_nodes = nodes.clone();
                    new_nodes.push(new_node.0);
                    q.push_back((new_nodes, cost + new_node.1))
                }
            }
        }

        *valid_paths.iter().max().unwrap() as u32
    }
}

impl From<&Grid> for Graph {
    fn from(grid: &Grid) -> Self {
        let nodes = grid.get_edge_contracted_nodes();
        let edges = grid.get_edges_from_edge_contracted_nodes(&nodes);

        Self { nodes: nodes, edges: edges }
    }
}
