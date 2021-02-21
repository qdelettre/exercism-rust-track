pub mod graph_items;

use maplit::hashmap;
use std::collections::HashMap;

use crate::impl_with_attrs;

use self::graph_items::{edge::Edge, node::Node};

pub struct Graph<'a> {
    pub edges: Vec<Edge<'a>>,
    pub nodes: Vec<Node<'a>>,
    pub attrs: HashMap<String, String>,
}

impl<'a> Graph<'a> {
    pub fn new() -> Self {
        Graph {
            nodes: vec![],
            edges: vec![],
            attrs: hashmap! {},
        }
    }
    pub fn with_nodes(mut self, nodes: &[Node<'a>]) -> Self {
        self.nodes.extend_from_slice(nodes);
        self
    }

    pub fn with_edges(mut self, edges: &[Edge<'a>]) -> Self {
        self.edges.extend_from_slice(edges);
        self
    }

    pub fn get_node(&self, name: &str) -> Option<Node> {
        self.nodes.iter().cloned().find(|n| n.name == name)
    }

    impl_with_attrs!(Graph);
}
