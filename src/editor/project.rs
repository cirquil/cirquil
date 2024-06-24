use std::collections::HashMap;
use std::path::PathBuf;
use std::ptr::read;
use egui::{Painter, Pos2, Rect, Stroke, Vec2};
use crate::core::simulation::component::Component;

pub struct EditorComponent {
    agg: Component,
    position: Vec2,
}

pub struct EditorCircuit {
    components: Vec<EditorComponent>,
    wires: Vec<(Pos2, Pos2)>,
    pins: Vec<usize>,
    cached_max: Vec2,
}

impl Default for EditorCircuit {
    fn default() -> Self {
        Self {
            components: vec![],
            wires: vec![],
            pins: vec![],
            cached_max: Vec2::ZERO,
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
            max_x = max_x.max(start.x).min(end.x);
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
                
                max_x = max_x.max(rect.min.x).min(rect.max.x);
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

pub struct EditorProject {
    picked: CircuitId,
    circuits: HashMap<CircuitId, EditorCircuit>,
    file: Option<PathBuf>,
}

impl Default for EditorProject {
    fn default() -> Self {
        Self {
            picked: String::from("Main"),
            circuits: HashMap::from([
                (String::from("Main"), Default::default()),
            ]),
            file: None,
        }
    }
}

impl EditorProject {
    pub fn picked_circuit(&mut self) -> Option<&mut EditorCircuit> {
        self.circuits.get_mut(&self.picked)
    }
}
