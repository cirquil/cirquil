use eframe::Frame;
use egui::{Color32, containers, Context, Pos2, Sense, Separator, Stroke, Vec2};
use egui_extras::{Size, StripBuilder};
use crate::core::canvas::CanvasCircuit;
use crate::core::circuit::Circuit;
use crate::core::value::Value;
use crate::gui::constants::MESH_STEP;
use crate::gui::mesh;

const MESH_SQUARE: Vec2 = Vec2::new(MESH_STEP, MESH_STEP);

pub struct CirquilApp {
    pub circuit: Circuit,
    pub canvas: CanvasCircuit,
}

impl eframe::App for CirquilApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            StripBuilder::new(ui)
                .size(Size::relative(0.15).at_least(50.0))
                .size(Size::exact(5.0))
                .size(Size::remainder())
                .horizontal(|mut strip| {
                    strip.cell(|ui| {
                        ui.centered_and_justified(|ui| {
                            if ui.button("Tick!").clicked() {
                                self.circuit.propagate();
                            };
                        });
                    });
                    strip.cell(|ui| {
                        ui.add(Separator::default().vertical());
                    });
                    strip.cell(|ui| {
                        containers::Frame::canvas(ui.style()).show(ui, |ui| {
                            let (mut response, painter) =
                                ui.allocate_painter(ui.available_size_before_wrap(), Sense::click());

                            mesh::draw(&response.rect, &painter);
                            let coords = response.rect.min.to_vec2();
                            for canvas_component in &self.canvas.components {
                                let component = self.circuit.get_component(canvas_component.component);
                                let mut component_coords = coords;
                                component_coords.x += canvas_component.loc.0 as f32;
                                component_coords.y += canvas_component.loc.1 as f32;
                                painter.extend(component.as_shapes(component_coords));
                            }

                            let inactive = Stroke::new(1.0, Color32::BLACK);
                            let active = Stroke::new(1.0, Color32::LIGHT_GREEN);
                            for canvas_wire in &self.canvas.wires {
                                let wire = self.circuit.get_wire(canvas_wire.wire);
                                for segment in &canvas_wire.segments {
                                    let (s, e) = segment;
                                    painter.line_segment(
                                        [Pos2::new(s.0 as f32, s.1 as f32) + coords, Pos2::new(e.0 as f32, e.1 as f32) + coords],
                                        if wire.value.get().get_raw_value() == 0 { inactive } else { active }
                                    );
                                }
                            }
                        });
                    });
                });
        });
    }
}
