use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct Edge {
    pub from: String,
    pub to: String,
}
#[derive(Debug, Deserialize)]
pub struct Node {
    pub id: String,
    pub name: String,
    // pub node_type: NodeType, 
}
#[derive(Debug, Deserialize)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}
// #[derive(Debug, Deserialize)]
// pub enum NodeType {
//     #[serde(rename = "Дуйсвие")]
//     Action,
//     #[serde(rename = "Ветвление")]
//     Branch,
//     #[serde(rename = "Конец")]
//     End,
// }
impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

}