use std::{env, fs};
use std::process::exit;

use egui::{Style, Visuals};

use cirquil::core::compiler::project::compile_project;
use cirquil::logisim::converter::convert_logisim_project;
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

            let mut logisim_project = convert_logisim_project(parse_logisim(filename).unwrap());
            // let saved_circuit = logisim_project.circuits.remove(&logisim_project.top_circuit).unwrap();
            // let (circuit, canvas) = compile_circuit(saved_circuit);
            // circuit.propagate_all();

            let compiled_circuits = compile_project(logisim_project);

            Box::new(CirquilPlayerApp { circuits: compiled_circuits, current_circuit: 0 })
        }),
    )
}
