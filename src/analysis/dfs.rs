use std::collections::{HashMap , HashSet};
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
