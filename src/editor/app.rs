use eframe::epaint::Shape;
use eframe::Frame;
use egui::Context;
use crate::editor::canvas::canvas_size;

use crate::editor::canvas::grid::ShapeExt;
use crate::editor::project::EditorProject;

use super::tools::{Action, Tree};

#[derive(Default)]
pub struct State {
    pub project: EditorProject,
}

#[derive(Default)]
pub struct CirquilEditor {
    state: State,
    tooling: Tree,
}

impl eframe::App for CirquilEditor {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        let sidebar = egui::SidePanel::left("sidebar")
            .resizable(true)
            .min_width(150.0);

        sidebar.show(ctx, |ui| {
            let mut tools = egui::TopBottomPanel::top("sidebar-tools");
            if let Some(r) = ctx.input(|i| i.viewport().inner_rect) {
                tools = tools.exact_height(r.size().y / 2.1);
            }
            tools.show_inside(ui, |ui| {
                self.tooling.show(ui)
            });

            let properties = egui::CentralPanel::default();
            properties.show_inside(ui, |_ui| {

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
                    
                    if let Some(tool) = self.tooling.picked_tool() {
                        tool.act(&mut self.state, &response, &painter, viewport);
                    } else {
                        eprintln!("Failed to retrieve the picked tool!");
                    }
                    
                    if let Some(circuit) = self.state.project.picked_circuit() {
                        circuit.show(&painter, viewport, response.rect.min.to_vec2());
                    } else {
                        eprintln!("Failed to retrieve the picked circuit!");
                    }
                });
            })
        });
    }
}
