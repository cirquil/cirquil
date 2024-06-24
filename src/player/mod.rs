use std::path::PathBuf;

use eframe::Error;
use egui::{Style, Visuals};

pub use app::CirquilPlayerApp;

pub mod app;
mod project;
mod file;
mod clock;
mod instrument;
pub mod osc;
mod workbench;
pub mod replay;
mod circuit;
pub mod probe_location;
mod csv;

pub fn run_player_gui(initial_project_file: Option<PathBuf>, initial_workbench_file: Option<PathBuf>) -> Result<(), Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Cirquil Player",
        options,
        Box::new(|cc| {
            let style = Style {
                visuals: Visuals::light(),
                ..Style::default()
            };
            cc.egui_ctx.set_style(style);

            Box::new(CirquilPlayerApp::from_file_options(initial_project_file, initial_workbench_file))
        }),
    )
}
