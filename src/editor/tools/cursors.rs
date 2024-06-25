use eframe::emath::Rect;
use egui::{Painter, PointerButton, Pos2, Response, Stroke};
use crate::editor::app::State;
use crate::editor::canvas::grid::{grid_normalize_end, nearest_grid_anchor};
use super::Action;

#[derive(Default)]
pub struct PointerCursor {
    dragged_component: Option<usize>,
}

impl Action for PointerCursor {
    fn act(&mut self, state: &mut State, response: &Response, _painter: &Painter, _viewport: Rect) {
        let Some(pointer) = response.ctx.pointer_hover_pos() else {
            return;
        };

        let Some(circuit) = state.project.picked_circuit() else {
            return eprintln!("Failed to retrieve the picked circuit!");
        };

        let mut components = state.drawn_circuit.components.iter()
            .filter(|(_, rect)| rect.contains(pointer))
            .rev();

        let wires = state.drawn_circuit.wires.iter()
            .filter(|(_, rect)| rect.contains(pointer))
            .rev();

        if response.clicked_by(PointerButton::Secondary) {
            wires.for_each(|(idx, _)| {
                circuit.wires.remove(*idx);
            });

            return components.for_each(|(idx, _)| {
                circuit.components.remove(*idx);
            });
        }

        let option_picked = components.next();
        
        if response.clicked() {
            state.picked_component = if let Some((idx, _)) = option_picked {
                Some(*idx)
            } else {
                None
            }
        }
        
        if response.drag_started() && option_picked.is_some() {
            let picked = option_picked.unwrap();
            self.dragged_component = Some(picked.0);
        }

        if let Some(idx) = self.dragged_component {
            circuit.components[idx].position += response.drag_delta();
        }

        if response.drag_released() && self.dragged_component.is_some() {
            let idx = self.dragged_component.take().unwrap();
            let pos = circuit.components[idx].position;
            circuit.components[idx].position = nearest_grid_anchor(pos.to_pos2()).to_vec2();
        }
    }
}

#[derive(Default)]
pub struct WireCursor {
    dragged_from: Option<Pos2>,
}

impl Action for WireCursor {
    fn act(&mut self, _state: &mut State, response: &Response, painter: &Painter, _viewport: Rect) {
        let Some(pointer) = response.ctx.pointer_hover_pos() else {
            return;
        };

        if response.clicked_by(PointerButton::Secondary) {
            return self.dragged_from = None;
        }

        let offset = response.rect.min.to_vec2();
        if !response.clicked() && self.dragged_from.is_some() {
            let start = self.dragged_from.unwrap() + offset;
            painter.line_segment(
                [start, grid_normalize_end(pointer, start)],
                Stroke::new(2.0, response.ctx.style().visuals.weak_text_color()),
            );
        }

        if !response.clicked() {
            return;
        }

        let end = if self.dragged_from.is_some() {
            let start = self.dragged_from.unwrap() + offset;
            let end = grid_normalize_end(pointer, start);

            if start == end {
                return self.dragged_from = None;
            }

            let Some(circuit) = _state.project.picked_circuit() else {
                return eprintln!("Failed to retrieve the picked circuit!");
            };

            circuit.add_wire(start - offset, end - offset);

            Some(end - offset)
        } else {
            None
        };

        self.dragged_from = end.or_else(|| {
            response.interact_pointer_pos().map(|pos| {
                nearest_grid_anchor(pos - offset)
            })
        });
    }
}
