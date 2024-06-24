use std::fmt::{Debug, Formatter};
use eframe::emath::Rect;
use egui::{Painter, Response, Ui};
use crate::editor::app::State;
use crate::editor::tools::{Action};

pub type ToolIdx = usize;

pub struct Tool {
    index: ToolIdx,
    name: String,
    action: Box<dyn Action>,
}

impl Debug for Tool {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tool")
            .field("index", &self.index)
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}

impl PartialEq for Tool {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl Eq for Tool {}

impl Action for Tool {
    fn act(&mut self, state: &mut State, response: &Response, painter: &Painter, viewport: Rect) {
        self.action.act(state, response, painter, viewport)
    }
}

impl Tool {
    pub fn new(index: ToolIdx, action: Box<dyn Action>, name: impl ToString) -> Self {
        Self { index, action, name: name.to_string() }
    }
    
    pub fn show(&self, current_pick: &mut ToolIdx, ui: &mut Ui) -> (&String, Response) {
        let is_picked = self.index == *current_pick;
        let response = ui.selectable_label(is_picked, &self.name);
        
        if response.clicked() {
            *current_pick = self.index;
        }
        
        (&self.name, response)
    }
    
    pub fn index(&self) -> ToolIdx {
        self.index
    }
}
