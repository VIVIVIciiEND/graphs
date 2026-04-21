mod Schedule;
mod analysis;
mod drawing;
mod export;
mod graph;
mod loarder;
mod node;
use crate::Schedule::open_schedule;
use crate::export::mermaid::export_mer;
use crate::graph::Graph;
use crate::graph::Node;
use analysis::dfs::{build_aw, build_dfs};
use eframe::egui;
use loarder::json_loader::load_graph;
use rfd::{AsyncFileDialog, FileDialog};
use std::{fmt::format, fs};

fn Name_node(nodes: &Vec<Node>, id: &str) -> String {
    for node in nodes {
        if node.id == id {
            return node.name.clone();
        }
    }
    return id.to_string();
}
struct GraphConstructor {
    graph: Option<Graph>,
    error_message: Option<String>,
    start_node: String,
    end_node: String,
    // depth: String ,
    found_paths: Vec<Vec<String>>,
    show_results: bool,
    export_fail: bool,
}
impl GraphConstructor {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            graph: None,
            error_message: None,
            start_node: String::new(),
            end_node: String::new(),
            found_paths: Vec::new(),
            show_results: false,
            export_fail: false,
        }
    }
}
impl eframe::App for GraphConstructor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                if ui.button("Загрузить файл .json").clicked() {
                    if let Some(path) = FileDialog::new().add_filter("json", &["json"]).pick_file()
                    {
                        match load_graph(path.to_str().unwrap()) {
                            Ok(graph) => {
                                self.graph = Some(graph);
                                self.error_message = None;
                            }
                            Err(e) => {
                                self.graph = None;
                                self.error_message = Some(format!("Ошибка загрузки {}", e))
                            }
                        }
                    }
                }
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(error) = &self.error_message {
                ui.colored_label(egui::Color32::RED, error);
            }
            ui.columns(2, |columns| {
                columns[0].heading("Параметры анализа");
                columns[0].add_space(5.0);
                egui::Frame::group(&ctx.style())
                    .fill(egui::Color32::from_rgb(40, 40, 40))
                    .show(&mut columns[0], |ui| {
                        ui.set_min_height(200.0);
                        ui.horizontal(|ui| {
                            ui.label("Начальный узел:");
                            ui.add(egui::TextEdit::singleline(&mut self.start_node).hint_text(""));
                        });
                        ui.add_space(5.0);
                        ui.horizontal(|ui| {
                            ui.label("Конечный узел:");
                            ui.add(egui::TextEdit::singleline(&mut self.end_node).hint_text(""));
                        });
                        ui.add_space(5.0);
                        if ui
                            .add(
                                egui::Button::new("Найти пути")
                                    .min_size(egui::vec2(ui.available_width(), 30.0)),
                            )
                            .clicked()
                        {
                            if let Some(graph) = &self.graph {
                                let aw1 = build_aw(graph);
                                let mut visidet = Vec::new();
                                let mut list_scenario = Vec::new();
                                build_dfs(
                                    self.start_node.clone(),
                                    &self.end_node,
                                    &aw1,
                                    &mut visidet,
                                    &mut list_scenario,
                                );
                                self.found_paths = list_scenario;
                                self.show_results = true;
                            } else {
                                self.show_results = true;
                                self.found_paths = Vec::new();
                            }
                        }
                        if ui
                            .add(
                                egui::Button::new("Экспорт файл mmd")
                                    .min_size(egui::vec2(ui.available_width(), 30.0)),
                            )
                            .clicked()
                        {
                            let mermaid = export_mer(&self.found_paths);
                            fs::write("graph.mmd", mermaid).expect("ошибка записи");
                            self.export_fail = true;
                        }
                        if self.export_fail {
                            ui.colored_label(egui::Color32::RED, "Файл загружен!");
                        }
                        if ui
                            .add(
                                egui::Button::new("Открыть файл mmd через html")
                                    .min_size(egui::vec2(ui.available_width(), 30.0)),
                            )
                            .clicked()
                        {
                            open_schedule("graph.mmd")
                        }
                    });
                columns[1].heading("РЕЗУЛЬТАТЫ");
                columns[1].add_space(5.0);
                egui::Frame::group(&ctx.style())
                    .fill(egui::Color32::from_rgb(30, 30, 30))
                    .show(&mut columns[1], |ui| {
                        ui.set_min_height(ui.available_height());
                        if self.show_results && !self.found_paths.is_empty() {
                            ui.label(format!("Найдено {} путей:", self.found_paths.len()));
                            ui.add_space(5.0);
                            egui::ScrollArea::vertical().show(ui, |ui| {
                                for (i, path) in self.found_paths.iter().enumerate() {
                                    if let Some(graph) = &self.graph {
                                        let mut path_names = Vec::new();
                                        for node_id in path {
                                            let name = Name_node(&graph.nodes, node_id);
                                            path_names.push(name);
                                        }
                                        ui.label(format!(
                                            "Путь {}: {}",
                                            i + 1,
                                            path_names.join(" -> ")
                                        ));
                                    }
                                }
                            });
                        } else if self.show_results {
                            ui.label("Пути не найдены");
                        }
                    });
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1400.0, 700.0)),
        min_window_size: Some(egui::vec2(800.0, 400.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Graph Constructor",
        options,
        Box::new(|cc| Box::new(GraphConstructor::new(cc))),
    )
}
