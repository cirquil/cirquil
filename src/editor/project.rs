use std::collections::HashMap;

use egui::{Painter, Pos2, Rect, Stroke, Vec2};

use crate::core::simulation::component::{Component, ComponentPins};
use crate::serde::project::{ProjectFile, SavedCircuit, SavedComponent};

#[derive(Debug)]
pub struct EditorComponent {
    agg: Component,
    position: Vec2,
}

impl From<SavedComponent> for EditorComponent {
    fn from(value: SavedComponent) -> Self {
        Self {
            agg: value.component,
            position: egui::vec2(value.location.x as f32, value.location.y as f32),
        }
    }
}

#[derive(Default, Debug)]
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
            wires: value.wires.into_iter().map(|wire| {(
                egui::pos2(wire.start.x as f32, wire.start.y as f32),
                egui::pos2(wire.end.x as f32, wire.end.y as f32),
            )}).collect(),
            pins: ComponentPins::new(value.pins.into_iter().map(From::from).collect()),
            cached_max: Default::default(),
        }
    }
}

impl EditorCircuit {
    pub fn add_wire(&mut self, start: Pos2, end: Pos2) {
        self.wires.push((start, end));
    }

    pub fn add_component(&mut self, component: Component, position: Vec2) {
        self.components.push(EditorComponent { agg: component, position })
    }
    
    pub fn show(&mut self, painter: &Painter, viewport: Rect, offset: Vec2) {
        let mut max_x = 0.0f32;
        let mut max_y = 0.0f32;
        
        let color = painter.ctx().style().visuals.text_color();
        let stroke = Stroke::new(2.0, color);
        for (start, end) in self.wires.iter() {
            max_x = max_x.max(start.x).max(end.x);
            max_y = max_y.max(start.y).max(end.y);
            
            if viewport.contains(*start) || viewport.contains(*end) {
                painter.line_segment([*start + offset, *end + offset], stroke);
            }
        }

        for component in self.components.iter() {
            let shapes = component.agg.as_shapes(painter.ctx());
            for mut shape in shapes {
                shape.translate(component.position);
                let rect = shape.visual_bounding_rect();
                let visible = viewport.intersects(shape.visual_bounding_rect());
                shape.translate(offset);

                if visible {
                    painter.add(shape);
                }
                
                max_x = max_x.max(rect.min.x).max(rect.max.x);
                max_y = max_y.max(rect.min.y).max(rect.max.y);
            }
        }
        
        self.cached_max = egui::vec2(max_x + 100.0, max_y + 100.0);
    }
    
    pub fn dimensions(&self) -> Vec2 {
        self.cached_max
    }
}

pub type CircuitId = String;

#[derive(Debug)]
pub struct EditorProject {
    picked: CircuitId,
    top: CircuitId,
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

impl EditorProject {
    pub fn pick(&mut self, id: &CircuitId) {
        if self.circuits.contains_key(id) {
            self.picked = id.clone();
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
