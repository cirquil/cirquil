use std::path::PathBuf;
use eframe::Frame;
use egui::{Context, Shape};

use crate::editor::canvas::canvas_size;
use crate::editor::canvas::grid::ShapeExt;
use crate::editor::project::{DrawnCircuit, EditorProject};
use crate::logisim::converter::convert_logisim_project;
use crate::logisim::parser::parse_logisim;
use crate::serde::project::ProjectFile;

use super::tools::{Action, Tree};

#[derive(Default)]
pub struct State {
    pub project: EditorProject,
    pub path: Option<PathBuf>,
    pub drawn_circuit: DrawnCircuit,
    pub picked_component: Option<usize>,
}

pub struct CirquilEditor {
    state: State,
    tooling: Tree,
}

impl Default for CirquilEditor {
    fn default() -> Self {
        let state = State::default();
        let mut tooling = Tree::default();

        tooling.populate_circuits(state.project.known_circuits());

        Self {
            state,
            tooling,
        }
    }
}

impl eframe::App for CirquilEditor {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        let menubar = egui::TopBottomPanel::top("menubar")
            .exact_height(20.0);

        menubar.show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    ui.menu_button("Open", |ui| {
                        if ui.button("Cirquil Project (.cirq)").clicked() {
                            ui.close_menu();

                            let option_project_path = rfd::FileDialog::new()
                                .add_filter("Cirquil Project", &["cirq"])
                                .pick_file();

                            if let Some(project_path) = option_project_path {
                                if let Ok(project_file) = ProjectFile::load(&project_path) {
                                    self.state.path = Some(project_path);
                                    self.state.project = From::from(project_file);
                                    self.tooling.populate_circuits(self.state.project.known_circuits());
                                }
                            }
                        };

                        if ui.button("Logisim Project (.circ)").clicked() {
                            ui.close_menu();

                            let option_logisim_project = rfd::FileDialog::new()
                                .add_filter("Logisim Project", &["circ"])
                                .pick_file()
                                .map(parse_logisim);

                            if let Some(Ok(logisim_project)) = option_logisim_project {
                                self.state.path = None;
                                self.state.project = From::from(convert_logisim_project(logisim_project));
                                self.tooling.populate_circuits(self.state.project.known_circuits());
                            }
                        };
                    });

                    if ui.add_enabled(self.state.path.is_some(), egui::Button::new("Save")).clicked() {
                        ui.close_menu();

                        let _ = ProjectFile::from(self.state.project.clone()).save(self.state.path.clone().unwrap());
                    }

                    if ui.button("Save As...").clicked() {
                        ui.close_menu();

                        let mut file_dialog = rfd::FileDialog::new()
                            .set_file_name(format!("{}.cirq", &self.state.project.top));

                        if let Ok(path) = std::env::current_dir() {
                            file_dialog = file_dialog.set_directory(path);
                        }

                        if let Some(path) = file_dialog.save_file() {
                            self.state.path = Some(path.clone());
                            let _ = ProjectFile::from(self.state.project.clone()).save(path);
                        }
                    }
                });

                ui.menu_button("Edit", |ui| {
                    if ui.button("Create Subcircuit").clicked() {
                        ui.close_menu();

                        self.tooling.populate_circuits(self.state.project.known_circuits());
                        todo!();
                    };
                });
            });
        });

        let sidebar = egui::SidePanel::left("sidebar")
            .resizable(true)
            .min_width(150.0);

        sidebar.show(ctx, |ui| {
            let mut tools = egui::TopBottomPanel::top("sidebar-tools");
            if let Some(r) = ctx.input(|i| i.viewport().inner_rect) {
                tools = tools.exact_height(r.size().y / 2.1);
            }
            tools.show_inside(ui, |ui| {
                ui.heading("Tooling");

                self.tooling.show(ui).into_iter().for_each(|(id, response)| {
                    if response.double_clicked() {
                        self.state.picked_component = None;
                        self.state.project.pick(id);
                    }

                    // TODO: add rename context menu
                });
            });

            let properties = egui::CentralPanel::default();
            properties.show_inside(ui, |ui| {
                ui.heading("Properties");

                let grid = egui::Grid::new("properties")
                    .num_columns(2)
                    .spacing([40.0, 4.0])
                    .striped(true);

                grid.show(ui, |ui| {
                    let Some(picked) = self.state.picked_component else { return; };
                    let Some(circuit) = self.state.project.picked_circuit() else { return; };
                    let Some(component) = circuit.components.get(picked) else { return; };

                    let c_properties = &component.agg.properties;
                    for property_name in c_properties.list() {
                        ui.label(property_name);

                        // let property = c_properties.get(property_name).unwrap();

                        ui.end_row();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().show_viewport(ui, |ui, viewport| {
                egui::Frame {
                    inner_margin: egui::Margin::ZERO,
                    ..egui::Frame::canvas(ui.style())
                }.show(ui, |ui| {
                    let canvas_size = canvas_size(viewport, self.state.project.picked_circuit().map(|r| &*r));
                    let (response, painter) = ui.allocate_painter(canvas_size, egui::Sense::click_and_drag());

                    let grid_viewport = viewport.translate(response.rect.min.to_vec2());
                    painter.extend(Shape::grid(grid_viewport, viewport.min.to_vec2(), &ctx.style()));

                    self.state.drawn_circuit = if let Some(circuit) = self.state.project.picked_circuit() {
                        circuit.show(&painter, viewport, response.rect.min.to_vec2())
                    } else {
                        eprintln!("Failed to retrieve the picked circuit!");
                        DrawnCircuit::default()
                    };

                    if let Some(tool) = self.tooling.picked_tool() {
                        tool.act(&mut self.state, &response, &painter, viewport);
                    } else {
                        eprintln!("Failed to retrieve the picked tool!");
                    }
                });
            })
        });
    }
}
