use crate::graph::Graph;
use std::fs;

pub fn load_graph(path: &str) -> Result<Graph, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(path)?;
    let graph: Graph = serde_json::from_str(&data)?;
    Ok(graph)
}