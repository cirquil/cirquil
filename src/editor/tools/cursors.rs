use eframe::emath::Rect;
use egui::{Painter, PointerButton, Pos2, Response, Stroke};
use crate::editor::app::State;
use crate::editor::canvas::grid::{grid_normalize_end, nearest_grid_anchor};
use super::Action;

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
