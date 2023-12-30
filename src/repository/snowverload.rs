use std::collections::HashMap;

use itertools::Itertools;
use rustworkx_core::petgraph::{graph::UnGraph, stable_graph::NodeIndex};

pub fn from_input(input: &str) -> UnGraph<&str, &str> {
    let mut graph: UnGraph<&str, &str> = UnGraph::new_undirected();

    let mut nodes: HashMap<&str, NodeIndex> = HashMap::new();
    let mut edges: Vec<(NodeIndex<u32>, NodeIndex<u32>)> = Vec::new();

    input.lines().map(|line| {
        let mut split = line.split(": ");
        let base_node_name = split.nth(0).unwrap();
        let base_node = get_node(&mut nodes, base_node_name, &mut graph);

        for new_node_name in split.nth(0).unwrap().split(" ") {
            let new_node = get_node(&mut nodes, new_node_name, &mut graph);
            edges.push((base_node, new_node));
        };
    }).collect_vec();

    graph.extend_with_edges(edges.iter());
    graph
}


fn get_node<'a>(nodes: &mut HashMap<&'a str, NodeIndex>, node_name: &'a str, graph: &mut UnGraph<&'a str, &'a str>) -> NodeIndex {
    match nodes.get(node_name) {
        Some(v) => *v,
        None => {
            let new_node = graph.add_node(node_name);
            nodes.insert(node_name, new_node);
            new_node
        },
    }
}