// use crate::graph::Graph;
// use eframe::egui::{self, Color32, Painter, Pos2};

// pub fn draw_graph(
//     graph: &Graph ,
//     painter: &Painter,
//     points: &std::collections::HashMap<String , (f32 , f32)>,

// ){
//     // egui::CentralPanel::default().show(ctx , |ui|{
//     //     let (response , painter) = ui.allocate_painter(
//     //         ui.available_size(),
//     //         egui::Sense::hover(),
//     //     );// получение кисти для рисование графа 
//     //     for (x,y) in points.iter().enumerate(){
//     //         painter.circle_filled(
//     //             egui::pos2(x , y),
//     //             20.0,
//     //             egui::Color32::from_rgb(255, 105, 180), 
//     //         )
//     //     }
//     // });
//     for edge in &graph.edges{
//         let from = points.get(&edge.from).unwrap();
//         let to = points.get(&edge.to).unwrap();
//         painter.line_segment(
//             [Pos2::new(from.0, from.1), Pos2::new(to.0 , to.1)],
//             eframe::egui::Stroke::new(3.0 , Color32::from_rgb(173, 255, 47),)
//         )
//     }
//     for node in &graph.nodes{
//         let c = points.get(&node.id).unwrap();
//         let p = Pos2::new(c.0 , c.1);
//         painter.circle_filled(p, 30.0, Color32::from_rgb(255, 105, 180));
//     painter.text(p, 
//         eframe::egui::Align2::CENTER_CENTER, 
//         &node.name, 
//         egui::FontId::default(), 
//         Color32::WHITE);
//     }
    
// }