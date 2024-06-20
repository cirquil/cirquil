use eframe::epaint::Shape;
use eframe::Frame;
use egui::{containers, Context, Pos2, Sense, Separator, Stroke, Vec2};
use egui_extras::{Size, StripBuilder};

use crate::core::canvas::circuit::CanvasCircuit;
use crate::core::simulation::circuit::Circuit;
use crate::gui::constants::GRID_STEP;
use crate::gui::grid;
use crate::gui::value::get_value_color;

const GRID_SQUARE: Vec2 = Vec2::new(GRID_STEP, GRID_STEP);

pub struct CirquilPlayerApp {
    pub circuit: Circuit,
    pub canvas: CanvasCircuit,
}

impl eframe::App for CirquilPlayerApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            StripBuilder::new(ui)
                .size(Size::exact(150.0))
                .size(Size::exact(5.0))
                .size(Size::remainder())
                .horizontal(|mut strip| {
                    strip.cell(|ui| {
                        ui.centered_and_justified(|ui| {
                            if ui.button("Tick!").clicked() {
                                self.circuit.tick();
                                self.circuit.propagate_ticked();
                            };
                        });
                    });
                    strip.cell(|ui| {
                        ui.add(Separator::default().vertical());
                    });
                    strip.cell(|ui| {
                        containers::Frame::canvas(ui.style()).show(ui, |ui| {

                            let (response, painter) =
                                ui.allocate_painter(ui.available_size_before_wrap(), Sense::drag());

                            grid::draw(&response.rect, &painter);
                            let coords = response.rect.min.to_vec2();

                            for canvas_wire in &self.canvas.wires {
                                let wire = self.circuit.get_wire(canvas_wire.wire);

                                let bit_width = if !wire.connected_components.is_empty() {
                                    let (component_idx, pin_idx) = wire.connected_components.first().unwrap();
                                    self.circuit.get_component(*component_idx).get_pins().get(*pin_idx).unwrap().bit_width
                                } else {
                                    1
                                };

                                let color = get_value_color(
                                    wire.value.get(),
                                    bit_width,
                                );

                                for segment in &canvas_wire.segments {
                                    let (s, e) = *segment;
                                    painter.line_segment(
                                        [Pos2::from(s) + coords, Pos2::from(e) + coords],
                                        Stroke::new(2f32, color),
                                    );
                                }
                                for node in &canvas_wire.nodes {
                                    painter.circle_filled(
                                        Pos2::from(*node) + coords, 3.5f32,
                                        color,
                                    );
                                }
                            }

                            for canvas_component in &self.canvas.components {
                                let component = self.circuit.get_component(canvas_component.component);
                                let component_coords = coords + Vec2::from(canvas_component.loc);

                                if let Some(mut interact_pos) = response.interact_pointer_pos() {
                                    interact_pos -= component_coords;
                                    if component.get_bounds().contains(interact_pos) {
                                        if response.drag_started() { component.mouse_pressed(interact_pos) }
                                        if response.drag_released() { component.mouse_released(interact_pos) }
                                        if response.clicked() { component.mouse_clicked(interact_pos) }
                                        if response.dragged() { component.mouse_dragged(response.drag_delta()) }

                                        self.circuit.propagate(vec![component]);
                                    }
                                }

                                let mut shapes = component.as_shapes(ctx);
                                for shape in shapes.iter_mut() {
                                    shape.translate(component_coords)
                                }

                                for pin in component.get_pins() {
                                    let pin_coords = component_coords + Vec2::from(pin.location);
                                    let color = get_value_color(pin.value.get(), pin.bit_width);

                                    shapes.push(Shape::circle_filled(
                                        pin_coords.to_pos2(),
                                        2f32,
                                        color,
                                    ));
                                }

                                painter.extend(shapes);
                            }
                        });
                    });
                });
        });
    }
}
