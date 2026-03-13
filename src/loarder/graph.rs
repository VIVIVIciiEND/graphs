use serde::Deserialize;
#[derive( Debug ,Deserialize)]
pub struct Edge {
    pub from : String , 
    pub to : String,
}
#[derive(Debug, Deserialize )]
pub struct Node{
    pub id: String,
    pub name: String , 
}
#[derive(Debug , Deserialize )]
pub struct Graph{
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}