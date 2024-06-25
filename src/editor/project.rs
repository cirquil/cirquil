use std::collections::HashMap;

use egui::{Painter, Pos2, Rect, Shape, Stroke, Vec2};
use crate::core::canvas::location::Location;

use crate::core::simulation::component::{Component, ComponentModel, ComponentPins};
use crate::gui::value::get_value_color;
use crate::serde::project::{ProjectFile, SavedCircuit, SavedCircuitBounds, SavedCircuitPin, SavedComponent, SavedWire};

#[derive(Debug, Clone)]
pub struct EditorComponent {
    pub agg: Component,
    pub position: Vec2,
}

impl From<SavedComponent> for EditorComponent {
    fn from(value: SavedComponent) -> Self {
        Self {
            agg: value.component,
            position: egui::vec2(value.location.x as f32, value.location.y as f32),
        }
    }
}

impl From<EditorComponent> for SavedComponent {
    fn from(value: EditorComponent) -> Self {
        Self {
            location: Location { x: value.position.x as i16, y: value.position.y as i16 },
            component: value.agg,
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct EditorCircuit {
    pub components: Vec<EditorComponent>,
    pub wires: Vec<(Pos2, Pos2)>,
    pub pins: ComponentPins,
    cached_max: Vec2,
}

impl From<SavedCircuit> for EditorCircuit {
    fn from(value: SavedCircuit) -> Self {
        Self {
            components: value.components.into_iter().map(From::from).collect(),
            wires: value.wires.into_iter().map(|wire| {
                (
                    egui::pos2(wire.start.x as f32, wire.start.y as f32),
                    egui::pos2(wire.end.x as f32, wire.end.y as f32),
                )
            }).collect(),
            pins: ComponentPins::new(value.pins.into_iter().map(From::from).collect()),
            cached_max: Default::default(),
        }
    }
}

impl From<EditorCircuit> for SavedCircuit {
    fn from(value: EditorCircuit) -> Self {
        let x = value.cached_max.x as i16;
        let y = value.cached_max.y as i16;

        let pins = value.components.clone().into_iter().map(|component| {
            match component.agg.model {
                ComponentModel::InputPin(p) => {
                    Some(SavedCircuitPin {
                        location: component.agg.pins.get_pins()[0].location,
                        label: component.agg.properties.get("label").unwrap().as_string().unwrap().get(),
                        bit_width: component.agg.pins.get_pins()[0].bit_width,
                        direction: component.agg.pins.get_pins()[0].direction,
                    })
                },
                ComponentModel::OutputPin(p) => {
                    Some(SavedCircuitPin {
                        location: component.agg.pins.get_pins()[0].location,
                        label: component.agg.properties.get("label").unwrap().as_string().unwrap().get(),
                        bit_width: component.agg.pins.get_pins()[0].bit_width,
                        direction: component.agg.pins.get_pins()[0].direction,
                    })
                }
                _ => None
            }
        }).filter(|p| p.is_some()).map(|p| p.unwrap()).collect();

        Self {
            components: value.components.into_iter().map(From::from).collect(),
            wires: value.wires.into_iter().map(|(start, end)| {
                SavedWire {
                    start: Location { x: start.x as i16, y: start.y as i16 },
                    end: Location { x: end.x as i16, y: end.y as i16 },
                }
            }).collect(),
            bounds: SavedCircuitBounds { start: Location { x: 0, y: 0 }, end: Location { x, y } },
            pins,
        }
    }
}

#[derive(Default, Clone)]
pub struct DrawnCircuit {
    pub wires: Vec<(usize, Rect)>,
    pub components: Vec<(usize, Rect)>,
}


impl EditorCircuit {
    pub fn add_wire(&mut self, start: Pos2, end: Pos2) {
        self.wires.push((start, end));
    }

    pub fn add_component(&mut self, component: Component, position: Vec2) {
        self.components.push(EditorComponent { agg: component, position })
    }

    pub fn show(&mut self, painter: &Painter, viewport: Rect, offset: Vec2) -> DrawnCircuit {
        let mut max_x = 0.0f32;
        let mut max_y = 0.0f32;

        let mut drawn_circuit = DrawnCircuit::default();

        let color = painter.ctx().style().visuals.text_color();
        let stroke = Stroke::new(2.0, color);
        for (idx, (start, end)) in self.wires.iter().enumerate() {
            max_x = max_x.max(start.x).max(end.x);
            max_y = max_y.max(start.y).max(end.y);

            if viewport.contains(*start) || viewport.contains(*end) {
                let shape = Shape::line_segment([*start + offset, *end + offset], stroke);
                drawn_circuit.wires.push((idx, shape.visual_bounding_rect().expand(2.5)));
                painter.add(shape);
            }
        }

        for (idx, component) in self.components.iter().enumerate() {
            let mut shape = Shape::Vec(component.agg.as_shapes(painter.ctx()));

            shape.translate(component.position);
            let rect = shape.visual_bounding_rect();
            let visible = viewport.intersects(shape.visual_bounding_rect());
            shape.translate(offset);

            if visible {
                drawn_circuit.components.push((idx, shape.visual_bounding_rect()));
                painter.add(shape);

                painter.extend(component.agg.get_pins().iter().map(|pin| {
                    let coords = Pos2::from(pin.location) + component.position + offset;
                    Shape::circle_filled(coords, 2.0, get_value_color(pin.value.get(), pin.bit_width))
                }));
            }

            max_x = max_x.max(rect.min.x).max(rect.max.x);
            max_y = max_y.max(rect.min.y).max(rect.max.y);
        }

        self.cached_max = egui::vec2(max_x + 100.0, max_y + 100.0);

        drawn_circuit
    }

    pub fn dimensions(&self) -> Vec2 {
        self.cached_max
    }
}

pub type CircuitId = String;

#[derive(Debug, Clone)]
pub struct EditorProject {
    picked: CircuitId,
    pub top: CircuitId,
    circuits: HashMap<CircuitId, EditorCircuit>,
}

impl Default for EditorProject {
    fn default() -> Self {
        Self {
            picked: String::from("Main"),
            top: String::from("Main"),
            circuits: HashMap::from([
                (String::from("Main"), Default::default()),
            ]),
        }
    }
}

impl From<ProjectFile> for EditorProject {
    fn from(value: ProjectFile) -> Self {
        Self {
            picked: value.top_circuit.clone(),
            top: value.top_circuit,
            circuits: value.circuits.into_iter().map(|(k, v)| (k, From::from(v))).collect(),
        }
    }
}

impl From<EditorProject> for ProjectFile {
    fn from(value: EditorProject) -> Self {
        Self {
            top_circuit: value.top,
            circuits: value.circuits.into_iter().map(|(k, v)| {
                (k, From::from(v))
            }).collect()
        }
    }
}

impl EditorProject {
    pub fn pick(&mut self, id: &CircuitId) {
        if self.circuits.contains_key(id) {
            self.picked.clone_from(id);
        } else {
            eprintln!("There is no circuit with name '{id}'!");
        }
    }

    pub fn known_circuits(&self) -> Vec<CircuitId> {
        self.circuits.keys().map(Clone::clone).collect()
    }

    pub fn is_picked(&self, id: &CircuitId) -> bool {
        self.picked == *id
    }

    pub fn picked_circuit(&mut self) -> Option<&mut EditorCircuit> {
        self.circuits.get_mut(&self.picked)
    }

    pub fn get_circuit(&mut self, id: &CircuitId) -> Option<&mut EditorCircuit> {
        self.circuits.get_mut(id)
    }
}
