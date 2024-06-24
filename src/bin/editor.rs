#![windows_subsystem = "windows"]

use eframe::{App, CreationContext};
use cirquil::editor::CirquilEditor;

fn create_app(cc: &CreationContext) -> Box<dyn App> {
    let style = egui::Style {
        visuals: egui::Visuals::light(),
        ..Default::default()
    };
    
    cc.egui_ctx.set_style(style);
    
    Box::new(CirquilEditor::default())
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        ..Default::default()
    };
    
    eframe::run_native("Cirquil Editor", options, Box::new(create_app))
}
