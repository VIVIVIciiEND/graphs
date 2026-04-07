use std::collections::HashMap;
use crate::graph::Graph; 
pub fn build_aw(graph: &Graph) -> HashMap<String, Vec<String>> {
    let mut aw:HashMap<String, Vec<String>>  = HashMap::new();
    for edge in &graph.edges {
        if aw.contains_key(&edge.from) {
            aw.get_mut(&edge.from).unwrap().push(edge.to.clone());
        } else {
            aw.insert(edge.from.clone(), vec![edge.to.clone()]);
        }
    }
    aw
}// задача функции привести структуру gRAPH В нужный вид haspmAP тип a: [d , c ...] для удобного поиска путей
pub fn build_dfs(
    node: String , 
    end_node: &String ,  
    aw: &HashMap<String , Vec<String>> , 
    visited: &mut Vec<String>,
    list_scenario: &mut Vec<Vec<String>> , 
    // start: &String , 
){
    visited.push(node.clone());
    if node == *end_node{ 
        list_scenario.push(visited.clone()); 
    } 
    else if let Some(n_option) = aw.get(&node){
        for next in n_option{
            if !visited.contains(next){// првоерка на цикл 
                build_dfs(next.clone() , end_node , aw, visited , list_scenario); 
            }
        }
    }
    visited.pop();
}
