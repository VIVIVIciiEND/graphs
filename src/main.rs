mod graph;
mod loarder;
mod drawing;
mod node;
use eframe::egui_glow::painter;
use loarder::json_loader::load_graph;
use crate::graph::Graph;
use eframe::egui;
use crate::drawing::draw_graph;
use crate::node::coordin::generate_circle;
struct GraphConstructor {
    graph: Option<Graph>, 
    error_message: Option<String>,
    start_node: String, 
    end_node: String,  
    found_paths: Vec<String>,
    show_results: bool,
}
impl GraphConstructor {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        match load_graph("data/examle.json") {
            Ok(graph) => Self {
                graph: Some(graph),
                error_message: None,
                start_node: String::from(""),
                end_node: String::from(""),
                found_paths: vec![
                    String::from(""),
                    String::from(""),
                ],
                show_results: true,
            },
            Err(e) => Self {
                graph: None,
                error_message: Some(format!("Ошибка загрузки: {}", e)),
                start_node: String::new(),
                end_node: String::new(),
                found_paths: Vec::new(),
                show_results: false,
            }
        }
    }
}

impl eframe::App for GraphConstructor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // egui::TopBottomPanel::top("menu").show(ctx, |ui| {
        // egui::menu::bar(ui, |ui| {
        //         ui.menu_button("☰", |ui| {
        //             if ui.button("Открыть").clicked() {
        //                 //открыть файл
        //             }
        //             if ui.button("Помощь").clicked() {
        //                // хз какой-то текст
        //             }
        //         });
        //     })    ;
        // });
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(error) = &self.error_message {
                ui.colored_label(egui::Color32::RED, error);
            }
            ui.columns(2, |columns| {
                columns[0].heading("Граф");
                columns[0].add_space(5.0);
                egui::Frame::dark_canvas(&ctx.style())
                    .stroke(egui::Stroke::new(3.0, egui::Color32::RED))
                    .show(&mut columns[0], |ui| {
                        ui.set_min_size(egui::vec2(450.0, 350.0));
                        egui::Frame::dark_canvas(&ctx.style())
                            .stroke(egui::Stroke::new(1.0, egui::Color32::WHITE))
                            .show(ui, |ui| {
                                ui.set_min_size(egui::vec2(400.0, 300.0));
                                if let Some(graph) = &self.graph 
                                {
                                    let (response , painter) = ui.allocate_painter(ui.available_size(),  egui::Sense::hover(),);
                                    let positions = generate_circle(graph);
                                    draw_graph(graph, &painter, &positions);
                                } else {
                                    ui.label("Граф не загружен");
                                }
                            }); 
                    });
                columns[1].heading("Параметры анализа");
                columns[1].add_space(5.0);
                egui::Frame::group(&ctx.style())
                    .fill(egui::Color32::from_rgb(40, 40, 40))
                    .show(&mut columns[1], |ui| {
                        ui.set_min_height(150.0);
                        //Начальный узел
                        ui.horizontal(|ui| {
                            ui.label("Начальный узел:");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.start_node)
                                    .hint_text(""),
                            );
                        });
                        ui.add_space(5.0);
                        //Конечный узел
                        ui.horizontal(|ui| {
                            ui.label("Конечный узел:");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.end_node)
                                    .hint_text(""),
                            );
                        });
                        ui.add_space(5.0);
                        //Кнопка запуска поиска путей
                        if ui
                            .add(
                                egui::Button::new("Найти пути")
                                .min_size(egui::vec2(ui.available_width(), 30.0))
                                )
                            .clicked()
                        {
                            self.show_results = true;
                        }
                    });
                columns[1].add_space(10.0);
                columns[1].heading("РЕЗУЛЬТАТЫ");
                columns[1].add_space(5.0);
                egui::Frame::group(&ctx.style())
                    .fill(egui::Color32::from_rgb(30, 30, 30))
                    .show(&mut columns[1], |ui| {
                        ui.set_min_height(200.0);
                        if self.show_results && !self.found_paths.is_empty() {
                            ui.label(format!("Найдено {} путей:", self.found_paths.len()));
                            ui.add_space(5.0);
                            egui::ScrollArea::vertical()
                                .max_height(150.0)
                                .show(ui, |ui| {
                                    for (i, path) in self.found_paths.iter().enumerate() {
                                        ui.horizontal(|ui| {
                                            ui.label(format!("Путь {}:", i + 1));
                                            ui.label(path);
                                        });
                                    }
                                });
                            if ui
                            .add(
                                egui::Button::new("экспорт")
                                .min_size(egui::vec2(ui.available_width(), 25.0))
                            )
                            .clicked()
                            {
                                //экспорт файла либо dot. либо mermaid
                            }
                        } else if self.show_results {
                            ui.label("Пути не найдены");
                        } else {
                            ui.label("Введите параметры и нажмите 'Найти пути'");
                        }
                    });
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1400.0, 700.0)),
        min_window_size: Some(egui::vec2(800.0, 500.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Graph Constructor",
        options,
        Box::new(|cc| Box::new(GraphConstructor::new(cc))),
    )
}
