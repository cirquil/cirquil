use std::fmt::{Debug, Formatter};
use eframe::emath::Rect;
use egui::{Painter, Response, Ui};
use crate::editor::app::State;
use crate::editor::tools::{Action};

#[derive(Debug)]
pub enum ToolKind {
    Cursor {
        name: String,
    },
    Component {
        name: String,
    },
    Subcircuit {
        index: usize,
    },
}

impl ToolKind {
    #[inline(always)]
    pub fn cursor(name: impl ToString) -> Self {
        Self::Cursor { name: name.to_string() }
    }

    #[inline(always)]
    pub fn component(name: impl ToString) -> Self {
        Self::Component { name: name.to_string() }
    }
    
    #[inline(always)]
    pub fn subcircuit(index: usize) -> Self {
        Self::Subcircuit { index }
    }
}

pub type ToolIdx = usize;

pub struct Tool {
    index: ToolIdx,
    action: Box<dyn Action>,
    kind: ToolKind,
}

impl Debug for Tool {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tool")
            .field("index", &self.index)
            .field("kind", &self.kind)
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
    pub fn new(index: ToolIdx, action: Box<dyn Action>, kind: ToolKind) -> Self {
        Self { index, action, kind }
    }
    
    pub fn show(&self, current_pick: &mut ToolIdx, ui: &mut Ui) -> Response {
        let is_picked = self.index == *current_pick;
        let response = match &self.kind {
            ToolKind::Cursor { name, .. } => ui.selectable_label(is_picked, name),
            ToolKind::Component { name, .. } => ui.selectable_label(is_picked, name),
            ToolKind::Subcircuit { index, .. } => ui.selectable_label(is_picked, format!("Component #{index}"))
        };
        
        if response.clicked() {
            *current_pick = self.index;
        }
        
        response
    }
    
    pub fn index(&self) -> ToolIdx {
        self.index
    }
}
