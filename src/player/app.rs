use std::cmp::max;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::rc::Rc;
use std::time::{Duration, Instant};

use eframe::epaint::Shape;
use eframe::Frame;
use egui::{Button, containers, Context, Pos2, ScrollArea, Sense, Separator, Stroke, Ui, Vec2, Vec2b};
use egui::collapsing_header::CollapsingState;
use egui_extras::{Size, StripBuilder};

use crate::core::canvas::circuit::CanvasCircuit;
use crate::core::canvas::location::Location;
use crate::core::compiler::project::{InstantiatedCircuits, SimulationTreeNode};
use crate::core::simulation::circuit::{Circuit, CircuitIdx};
use crate::core::simulation::probe::{CanvasProbe, Probe};
use crate::gui::component::AsShapes;
use crate::gui::constants::GRID_STEP;
use crate::gui::grid;
use crate::gui::value::get_value_color;
use crate::player::clock::{ClockState, SimulationTicker};
use crate::player::file::OpenedFile;
use crate::player::instrument::Instrument;
use crate::player::osc::{draw_osc, Oscilloscope};
use crate::player::project::show_load_project_file_dialog;
use crate::player::workbench::{show_load_workbench_file_dialogue, show_save_workbench_file_dialogue};

const _GRID_SQUARE: Vec2 = Vec2::new(GRID_STEP, GRID_STEP);

const BUTTON_SIZE: Vec2 = Vec2::new(40.0, 40.0);

pub struct CirquilPlayerApp {
    pub circuits: InstantiatedCircuits,
    pub current_circuit: CircuitIdx,
    pub top_circuit: CircuitIdx,
    pub osc_visible: bool,
    pub record_armed: bool,
    pub project_file: OpenedFile,
    pub simulation_ticker: SimulationTicker,
    pub clock_state: ClockState,
    pub probes: Vec<CanvasProbe>,
    pub probe_max_id: usize,
    pub workbench_file: OpenedFile,
    pub current_instrument: Instrument,
    pub osc: Oscilloscope,
    pub failed_probe_errors: Option<Vec<String>>,
}

impl CirquilPlayerApp {
    pub fn new() -> Self {
        Self::from_file_option::<PathBuf>(None)
    }

    pub fn new_with_file<P>(initial_file: P) -> Self
        where
            P: AsRef<Path>,
    {
        Self::from_file_option(Some(initial_file))
    }

    fn from_file_option<P>(initial_file: Option<P>) -> Self
        where
            P: AsRef<Path>,
    {
        Self {
            circuits: InstantiatedCircuits {
                canvas_circuits: vec![CanvasCircuit {
                    name: "main".to_string(),
                    components: vec![],
                    wires: vec![],
                    appearance: (),
                    pins: (),
                }],
                instantiated_circuits: vec![
                    (
                        Rc::new(Circuit {
                            components: vec![],
                            wires: vec![],
                            clock_generators: vec![],
                            input_pins: vec![],
                            output_pins: vec![],
                        }),
                        0
                    ),
                ],
                simulation_tree: SimulationTreeNode::Leaf(0),
                by_uuid: vec![],
                parents: vec![],
            },
            current_circuit: 0,
            top_circuit: 0,
            osc_visible: false,
            record_armed: false,
            project_file: OpenedFile::new(initial_file.map(|x| PathBuf::from(x.as_ref()))),
            simulation_ticker: SimulationTicker {
                clock_speed: 1,
                clock_period: Duration::from_micros(1_000_000),
                timer: Instant::now(),
                tick_needed: false,
            },
            clock_state: ClockState::Stopped,
            probes: vec![],
            probe_max_id: 0,
            workbench_file: OpenedFile::new(None),
            current_instrument: Instrument::None,
            osc: Oscilloscope::default(),
            failed_probe_errors: None,
        }
    }
}

impl Default for CirquilPlayerApp {
    fn default() -> Self {
        Self::from_file_option::<PathBuf>(None)
    }
}

impl eframe::App for CirquilPlayerApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        if self.clock_state == ClockState::Running {
            ctx.request_repaint_after(self.simulation_ticker.clock_period);
        }

        if let Some(path) = self.project_file.check_load() {
            self.load_project(path).unwrap();
        }

        let (top_circuit, _) = self.circuits.instantiated_circuits.get(self.top_circuit).unwrap();

        if self.simulation_ticker.check_tick_needed() {
            self.tick(top_circuit);

            self.osc.collect_probe_values(self.probes.as_slice(), &self.circuits);
        }

        if let Some(failed_probe_errors) = &self.failed_probe_errors {
            let mut should_clear_errors = false;

            egui::Window::new("Workbench Errors")
                .min_width(500.0)
                .resizable(false)
                .collapsible(false)
                .show(ctx, |ui| {
                    ui.heading("Some probes loaded with errors: ");

                    for error in failed_probe_errors {
                        ui.label((*error).as_str());
                    }

                    ui.separator();

                    if ui.button("Ok").clicked() {
                        should_clear_errors = true;
                    }
                });

            if should_clear_errors {
                self.failed_probe_errors = None;
            }
        }

        egui::TopBottomPanel::top("menu_panel").exact_height(20.0).show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open project").clicked() {
                        if let Some(path) = show_load_project_file_dialog() {
                            self.project_file.request_open(path);
                        }

                        ui.close_menu()
                    };

                    ui.add(Separator::default().horizontal());

                    if ui.button("Open workbench").clicked() {
                        if let Some(path) = show_load_workbench_file_dialogue() {
                            self.failed_probe_errors = self.load_workbench(path);
                        }

                        ui.close_menu();
                    }

                    if ui.button("Save workbench").clicked() {
                        if let Some(path) = show_save_workbench_file_dialogue() {
                            self.save_workbench(path);
                        }

                        ui.close_menu();
                    }

                    ui.add(Separator::default().horizontal());

                    if ui.button("Quit").clicked() {
                        exit(0);
                    }
                });
            });
        });

        egui::TopBottomPanel::top("top_panel").exact_height(50.0).show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.horizontal(|ui| {
                    if ui.add(Button::new("Open project").min_size(BUTTON_SIZE)).clicked() {
                        if let Some(path) = show_load_project_file_dialog() {
                            self.project_file.request_open(path);
                        }
                    };
                    if ui.add(Button::new("Open workbench").min_size(BUTTON_SIZE)).clicked() {
                        if let Some(path) = show_load_workbench_file_dialogue() {
                            self.failed_probe_errors = self.load_workbench(path);
                        }
                    }

                    ui.add(Separator::default().vertical());

                    if ui.add(Button::new("Reset circuit").min_size(BUTTON_SIZE)).clicked() && self.project_file.current_file.is_some() {
                        self.project_file.request_open(self.project_file.current_file.clone().unwrap());
                    }

                    ui.add(Separator::default().vertical());

                    if ui.add(Button::new("Record").min_size(BUTTON_SIZE).selected(self.record_armed)).clicked() {
                        self.record_armed = !self.record_armed;
                    }

                    if ui.add_enabled(self.clock_state == ClockState::Running, Button::new("Stop").min_size(BUTTON_SIZE)).clicked() {
                        self.clock_state = ClockState::Stopped;
                    }
                    if ui.add_enabled(self.clock_state == ClockState::Stopped, Button::new("Play").min_size(BUTTON_SIZE)).clicked() {
                        self.clock_state = ClockState::Running;
                    }

                    if ui.add_enabled(self.clock_state == ClockState::Stopped, Button::new("Tick").min_size(BUTTON_SIZE)).clicked() {
                        self.simulation_ticker.request_tick();
                    }

                    ui.add(egui::Slider::new(&mut self.simulation_ticker.clock_speed, 1..=100).text("Clock speed (Hz)"));
                    self.simulation_ticker.clock_period = Duration::from_micros(1_000_000 / self.simulation_ticker.clock_speed);

                    if self.clock_state == ClockState::Running
                        && (self.simulation_ticker.timer.elapsed() > self.simulation_ticker.clock_period) {
                        self.simulation_ticker.request_tick();

                        self.simulation_ticker.timer = Instant::now();
                    }

                    ui.add(Separator::default().vertical());

                    if ui.add(Button::new("Probe").min_size(BUTTON_SIZE).selected(self.current_instrument == Instrument::Probe)).clicked() {
                        self.current_instrument = match &self.current_instrument {
                            Instrument::None => Instrument::Probe,
                            Instrument::Probe => Instrument::None,
                        }
                    }

                    if ui.add(Button::new("Osc").min_size(BUTTON_SIZE)).clicked() {
                        self.osc_visible = !self.osc_visible;
                    }
                })
            })
        });

        egui::SidePanel::left("left_panel")
            .resizable(false)
            .exact_width(150.0)
            .show(ctx, |ui| {
                StripBuilder::new(ui)
                    .size(Size::relative(0.5))
                    .size(Size::exact(15.0))
                    .size(Size::remainder())
                    .vertical(|mut strip| {
                        strip.cell(|ui| {
                            ui.heading("Simulation tree");

                            ScrollArea::vertical().id_source("simulation_tree_scroll").auto_shrink(Vec2b::new(false, false)).show(ui, |ui| {
                                if let Some(i) = traverse_simulation_tree(ui, &self.circuits.simulation_tree, &self.circuits, self.current_circuit) {
                                    self.current_circuit = i;
                                }
                            });
                        });

                        strip.cell(|ui| {
                            ui.centered_and_justified(|ui| {
                                ui.add(Separator::default().horizontal());
                            });
                        });

                        strip.cell(|ui| {
                            ui.heading("Probes");

                            ScrollArea::vertical().id_source("probes_scroll").auto_shrink(Vec2b::new(false, false)).show(ui, |ui| {
                                traverse_probes(ui, &mut self.probes, &mut self.current_circuit);
                            });
                        })
                    });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Window::new("Oscilloscope")
                .min_size(Vec2::new(600.0, 300.0))
                .max_size(Vec2::new(1400.0, 600.0))
                .open(&mut self.osc_visible)
                .show(ctx, |ui| draw_osc(ui, &mut self.osc, self.probes.as_slice()));

            ScrollArea::both().id_source("canvas_scroll").show(ui, |ui| {
                containers::Frame::canvas(ui.style()).show(ui, |ui| draw_canvas(ui, ctx, self.current_circuit, &self.circuits, &mut self.probes, &mut self.probe_max_id, &self.current_instrument));
            });
        });
    }
}

fn traverse_probes(ui: &mut Ui, probes: &mut Vec<CanvasProbe>, current_circuit: &mut CircuitIdx) {
    let mut remove_idx = None;

    for (idx, CanvasProbe { probe, .. }) in probes.iter_mut().enumerate() {
        let label = ui.selectable_label(false, probe.name.as_str());

        if label.clicked() {
            *current_circuit = probe.circuit;
        }

        label.context_menu(|ui| {
            ui.text_edit_singleline(&mut probe.name);

            if ui.button("Remove").clicked() {
                remove_idx = Some(idx);

                ui.close_menu();
            }
        });
    }

    if let Some(idx) = remove_idx {
        probes.remove(idx);
    }
}

fn traverse_simulation_tree(ui: &mut Ui, node: &SimulationTreeNode, circuits: &InstantiatedCircuits, current_circuit: CircuitIdx) -> Option<CircuitIdx> {
    let mut clicked_circuit = None;

    match node {
        SimulationTreeNode::Leaf(l) => {
            if ui.selectable_label(*l == current_circuit, circuits.get_circuit_name(*l)).clicked() {
                clicked_circuit = Some(*l);
            }
        }
        SimulationTreeNode::Node(i, ch) => {
            CollapsingState::load_with_default_open(ui.ctx(), ui.next_auto_id(), true)
                .show_header(ui, |ui| {
                    if ui.selectable_label(*i == current_circuit, circuits.get_circuit_name(*i)).clicked() {
                        clicked_circuit = Some(*i);
                    }
                })
                .body(|ui| {
                    for c in ch {
                        clicked_circuit = traverse_simulation_tree(ui, c, circuits, current_circuit).or(clicked_circuit);
                    }
                });
        }
    };

    clicked_circuit
}

fn calculate_canvas_bounds(canvas: &CanvasCircuit) -> Vec2 {
    let max_component_x = canvas.components.iter()
        .max_by(|a, b| a.loc.x.cmp(&b.loc.x))
        .map(|comp| comp.loc.x * 2)
        .unwrap_or(1000);

    let max_component_y = canvas.components.iter()
        .max_by(|a, b| a.loc.x.cmp(&b.loc.x))
        .map(|comp| comp.loc.y * 2)
        .unwrap_or(1000);

    let max_coord = max(max_component_x, max_component_y);

    Vec2::new(max_coord as f32, max_coord as f32)
}

fn draw_canvas(ui: &mut Ui, ctx: &Context, current_circuit: CircuitIdx, instantiated_circuits: &InstantiatedCircuits, probes: &mut Vec<CanvasProbe>, probe_id: &mut usize, current_instrument: &Instrument) {
    let (circuit, canvas_idx) = instantiated_circuits.instantiated_circuits.get(current_circuit).unwrap();
    let canvas = instantiated_circuits.canvas_circuits.get(*canvas_idx).unwrap();

    let canvas_bounds = calculate_canvas_bounds(canvas);

    let (response, painter) =
        ui.allocate_painter(canvas_bounds, Sense::click_and_drag());

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
                Stroke::new(2.0, color),
            );
        }
        for node in &canvas_wire.nodes {
            painter.circle_filled(
                Pos2::from(*node) + coords, 3.5,
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
                2.0,
                color,
            ));
        }

        painter.extend(shapes);
    }

    if *current_instrument == Instrument::Probe && response.clicked() {
        if let Some(mut interact_pos) = response.interact_pointer_pos() {
            interact_pos -= coords;

            let margin = 10;

            if let Some(wire) = canvas.wires.iter()
                .find(|wire| wire.contains(Location::from(interact_pos), margin))
            {
                let mut probe_location = Location::from(interact_pos);

                probe_location.x = probe_location.x - (probe_location.x % (2 * margin)) + margin;
                probe_location.y = probe_location.y - (probe_location.y % (2 * margin)) + margin;

                let probe_name = format!("probe_{}", probe_id);

                probes.push(
                    CanvasProbe {
                        location: probe_location,
                        probe: Probe {
                            name: probe_name.clone(),
                            circuit: current_circuit,
                            wire: wire.wire,
                        },
                    }
                );

                *probe_id += 1;
            }
        }
    }

    for CanvasProbe { location, probe } in probes {
        if current_circuit == probe.circuit {
            let mut shapes = probe.as_shapes(ctx);
            for shape in shapes.iter_mut() {
                shape.translate(coords + Vec2::from(*location))
            }

            painter.extend(shapes);
        }
    }
}
