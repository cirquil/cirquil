use eframe::Frame;
use egui::{Color32, containers, Context, Pos2, Sense, Separator, Stroke, Vec2};
use egui_extras::{Size, StripBuilder};
use crate::core::canvas::circuit::CanvasCircuit;

use crate::core::simulation::circuit::Circuit;
use crate::gui::constants::GRID_STEP;
use crate::gui::grid;

const GRID_SQUARE: Vec2 = Vec2::new(GRID_STEP, GRID_STEP);

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

                            grid::draw(&response.rect, &painter);
                            let coords = response.rect.min.to_vec2();
                            for canvas_component in &self.canvas.components {
                                let component = self.circuit.get_component(canvas_component.component);
                                let component_coords = coords + Vec2::from(canvas_component.loc);

                                let mut shapes = component.as_shapes();
                                for shape in shapes.iter_mut() {
                                    shape.translate(component_coords)
                                }

                                painter.extend(shapes);
                            }

                            let inactive = Stroke::new(2.0, Color32::DARK_GREEN);
                            let active = Stroke::new(2.0, Color32::LIGHT_GREEN);
                            for canvas_wire in &self.canvas.wires {
                                let wire = self.circuit.get_wire(canvas_wire.wire);
                                for segment in &canvas_wire.segments {
                                    let (s, e) = *segment;
                                    painter.line_segment(
                                        [Pos2::from(s) + coords, Pos2::from(e) + coords],
                                        if wire.value.get().get_raw_value() == 0 { inactive } else { active },
                                    );
                                }
                            }
                        });
                    });
                });
        });
    }
}
