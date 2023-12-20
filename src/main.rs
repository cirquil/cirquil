use crate::gui::CirquilApp;
use crate::logisim::converter::convert_circuit;
use crate::logisim::parser::parse_logisim;

mod core;
mod gui;
mod logisim;
mod test_propagate;

fn main() -> Result<(), eframe::Error> {
    let filename = "test_3.circ".to_string();
    // test_not();
    // test_propagate();
    // test_or();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Cirquil",
        options,
        Box::new(|cc| {
            let (circuit, canvas) = convert_circuit(parse_logisim(filename), 0);
            Box::new(CirquilApp { circuit, canvas })
        }),
    )
}
