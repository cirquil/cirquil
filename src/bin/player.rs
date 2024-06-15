use std::{env, fs};
use std::process::exit;

use egui::{Style, Visuals};

use cirquil::logisim::converter::convert_circuit;
use cirquil::logisim::parser::parse_logisim;
use cirquil::player::CirquilPlayerApp;

fn main() -> Result<(), eframe::Error> {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).unwrap_or(&"test.circ".to_string()).clone();
    
    if let Err(err) = fs::metadata(&filename) {
        println!("{}: {}", err, filename);
        exit(1);
    }

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

            let (circuit, canvas) = convert_circuit(parse_logisim(filename), 0);
            circuit.propagate_all();
            Box::new(CirquilPlayerApp { circuit, canvas })
        }),
    )
}
