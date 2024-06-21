#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;

use egui::{Style, Visuals};

use cirquil::player::CirquilPlayerApp;

fn main() -> Result<(), eframe::Error> {
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

            let args: Vec<String> = env::args().collect();

            let filename = args.get(1);

            match filename {
                Some(filename) => {
                    Box::new(CirquilPlayerApp::new_with_file(filename))
                }
                None => {
                    Box::new(CirquilPlayerApp::new())
                }
            }
        }),
    )
}
