// use crate::graph::Graph;
// use std::f32::consts::PI;
// use std::collections::HashMap;

// pub fn generate_circle(graph: &Graph) -> HashMap<String , (f32 , f32)>{
//     let r = 150.0 ; 
//     let centr_x = 400.0/1.5;
//     let centre_y = 300.0/1.0;
//     let mut points = HashMap::new();
//     let num_points = graph.nodes.len();
//     for (i, node)  in graph.nodes.iter().enumerate() {
//         let angle = 2.0 * PI * i as f32 / num_points as f32;
//         let x = centr_x + r * angle.cos();
//         let y = centre_y + r * angle.sin();
//         points.insert(node.id.clone(), (x , y));
//     }
//     points 
// }