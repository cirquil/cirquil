use std::process::exit;

use eframe::epaint::Shape;
use eframe::Frame;
use egui::{Button, containers, Context, Pos2, Sense, Separator, Stroke, Ui, Vec2};

use crate::core::canvas::circuit::CanvasCircuit;
use crate::core::compiler::project::InstantiatedCircuits;
use crate::core::simulation::circuit::{Circuit, CircuitIdx};
use crate::gui::constants::GRID_STEP;
use crate::gui::grid;
use crate::gui::value::get_value_color;

const GRID_SQUARE: Vec2 = Vec2::new(GRID_STEP, GRID_STEP);

const BUTTON_SIZE: Vec2 = Vec2::new(40.0, 40.0);

pub struct CirquilPlayerApp {
    pub circuits: InstantiatedCircuits,
    pub current_circuit: CircuitIdx,
    pub osc_visible: bool,
}

impl eframe::App for CirquilPlayerApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        let (circuit, canvas_circuit_idx) = self.circuits.instantiated_circuits.get(self.current_circuit).unwrap();
        let canvas = self.circuits.canvas_circuits.get(*canvas_circuit_idx).unwrap();

        egui::TopBottomPanel::top("menu_panel").min_height(20.0).show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    let _ = ui.button("Open project");
                    let _ = ui.button("Open workbench");

                    ui.add(Separator::default().horizontal());

                    if ui.button("Quit").clicked() {
                        exit(0);
                    }
                });
            });
        });

        egui::TopBottomPanel::top("top_panel").min_height(50.0).show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.horizontal(|ui| {
                    ui.add(Button::new("Open project").min_size(BUTTON_SIZE));
                    ui.add(Button::new("Open workbench").min_size(BUTTON_SIZE));

                    ui.add(Separator::default().vertical());

                    ui.add(Button::new("Record").min_size(BUTTON_SIZE));
                    ui.add_enabled(false, Button::new("Stop").min_size(BUTTON_SIZE));
                    ui.add(Button::new("Play").min_size(BUTTON_SIZE));
                    if ui.add(Button::new("Tick").min_size(BUTTON_SIZE)).clicked() {
                        circuit.tick();
                        circuit.propagate_ticked();
                    }

                    ui.add(Separator::default().vertical());

                    if ui.add(Button::new("Change circuit").min_size(BUTTON_SIZE)).clicked() {
                        self.current_circuit += 1;

                        if self.current_circuit >= self.circuits.instantiated_circuits.len() {
                            self.current_circuit = 0;
                        }
                    }
                    if ui.add(Button::new("Osc").min_size(BUTTON_SIZE)).clicked() {
                        self.osc_visible = !self.osc_visible;
                    }
                })
            })
        });

        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            ui.heading("Simulation tree");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Window::new("Oscilloscope").open(&mut self.osc_visible).show(ctx, draw_osc);
            containers::Frame::canvas(ui.style()).show(ui, |ui| draw_canvas(ui, ctx, canvas, circuit));
        });
    }
}

fn draw_osc(ui: &mut Ui) {
    ui.label("I am Osc");
}

fn draw_canvas(ui: &mut Ui, ctx: &Context, canvas: &CanvasCircuit, circuit: &Circuit) {
    let (response, painter) =
        ui.allocate_painter(ui.available_size_before_wrap(), Sense::click_and_drag());

    grid::draw(&response.rect, &painter);
    let coords = response.rect.min.to_vec2();

    for canvas_wire in canvas.wires.iter() {
        let wire = circuit.get_wire(canvas_wire.wire);

        let bit_width = if !wire.connected_components.is_empty() {
            let (component_idx, pin_idx) = wire.connected_components.first().unwrap();
            circuit.get_component(*component_idx).get_pins().get(*pin_idx).unwrap().bit_width
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

    for canvas_component in canvas.components.iter() {
        let component = circuit.get_component(canvas_component.component);
        let component_coords = coords + Vec2::from(canvas_component.loc);

        if let Some(mut interact_pos) = response.interact_pointer_pos() {
            interact_pos -= component_coords;
            if component.get_bounds().contains(interact_pos) {
                if response.drag_started() { component.mouse_pressed(interact_pos) }
                if response.drag_released() { component.mouse_released(interact_pos) }
                if response.clicked() { component.mouse_clicked(interact_pos) }
                if response.dragged() { component.mouse_dragged(response.drag_delta()) }

                circuit.propagate(vec![component]);
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
}
