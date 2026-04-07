use std::collections::{HashMap , HashSet};

fn build_aw(graph: &Graph) -> HashMap<String, Vec<String>> {
    let mut aw:HashMap<String, Vec<String>>  = HashMap::new();
    for edge in &graph.edges {
        if aw.contains_key(&edge.from) {
            aw.get_mut(&edge.from).unwrap().push(edge.to.clone());
        } else {
            aw.insert(edge.from.clone(), vec![edge.to.clone()]);
        }
    }
    aw
}
pub fn dfs(
    node: &String , 
    graph: &HashMap<String , Vec<String>>,
    visited: &mut HashSet<String>,
    
){
    visited.insert(node.clone());
    let n_option = graph.get(node);
    if n_option.is_some(){
        let n = n_option.unwrap();
        for i in n {
            if visited.insert(i.clone){
                dfs(i , graph , visited)
        }
    }
}
}
