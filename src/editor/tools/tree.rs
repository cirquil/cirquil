use egui::{InnerResponse, Response, Ui};
use egui::collapsing_header::CollapsingState;
use crate::core::simulation::components::clock_generator::ClockGenerator;
use crate::core::simulation::components::input::button::InputButton;
use crate::core::simulation::components::logic::and_gate::AndGate;
use crate::core::simulation::components::logic::not_gate::NotGate;
use crate::core::simulation::components::logic::or_gate::OrGate;
use crate::core::simulation::components::tunnel::Tunnel;
use crate::editor::tools::component::ComponentFactory;
use crate::editor::tools::cursors::WireCursor;
use super::tool::{Tool, ToolIdx, ToolKind};

#[derive(Debug)]
pub struct Group {
    name: String,
    tools: Vec<Tool>,
}

impl Group {
    #[inline(always)]
    fn new(name: &str, tools: Vec<Tool>) -> Self {
        Group { name: String::from(name), tools }
    }

    fn show(&self, current_pick: &mut ToolIdx, ui: &mut Ui) -> (Response, InnerResponse<Response>, Option<InnerResponse<Vec<Response>>>) {
        let state = CollapsingState::load_with_default_open(ui.ctx(), ui.next_auto_id(), true);
        state.show_header(ui, |ui| {
            ui.label(&self.name)
        }).body(|ui| {
            Vec::from_iter(self.tools.iter().map(|tool| {
                tool.show(current_pick, ui)
            }))
        })
    }
}

#[derive(Debug)]
pub struct Tree {
    picked: ToolIdx,
    groups: Vec<Group>,
}

impl Default for Tree {
    fn default() -> Self {
        let groups = vec![
            Group::new("Tools", vec![
                Tool::new(0, Box::new(WireCursor::default()), ToolKind::cursor("Wire")),
            ]),
            Group::new("Wiring", vec![
                Tool::new(1, Box::new(ComponentFactory::new(Box::new(|| {
                    Tunnel::from_name_width("tunnel", 8)
                }))), ToolKind::component("Tunnel")),
                Tool::new(2, Box::new(ComponentFactory::new(Box::new(|| {
                    ClockGenerator::create()
                }))), ToolKind::component("Clock Generator")),
            ]),
            Group::new("Gates", vec![
                Tool::new(3, Box::new(ComponentFactory::new(Box::new(|| {
                    NotGate::from_bit_width(8)
                }))), ToolKind::cursor("NOT Gate")),
                Tool::new(4, Box::new(ComponentFactory::new(Box::new(|| {
                    AndGate::from_bit_width(8)
                }))), ToolKind::cursor("AND Gate")),
                Tool::new(5, Box::new(ComponentFactory::new(Box::new(|| {
                    OrGate::from_bit_width(8)
                }))), ToolKind::cursor("OR Gate")),
            ]),
            Group::new("Input and Output", vec![
                Tool::new(6, Box::new(ComponentFactory::new(Box::new(|| {
                    InputButton::create()
                }))), ToolKind::cursor("Button")),
            ]),
        ];

        Tree {
            picked: 0,
            groups,
        }
    }
}

impl Tree {
    pub fn show(&mut self, ui: &mut Ui) {
        Vec::from_iter(self.groups.iter().map(|group| {
            group.show(&mut self.picked, ui)
        }));
    }
    
    pub fn picked_tool(&mut self) -> Option<&mut Tool> {
        let mut picked_tool = None;
        
        for group in self.groups.iter_mut() {
            for tool in group.tools.iter_mut() {
                if tool.index() == self.picked {
                    picked_tool = Some(tool);
                    break
                }
            }
        }

        picked_tool
    }
}
