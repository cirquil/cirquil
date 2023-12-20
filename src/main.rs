use crate::test_canvas::provide_circuit;
use crate::test_propagate::{test_not, test_or, test_propagate};
use crate::gui::CirquilApp;

mod core;
mod gui;
mod logisim;
mod test_propagate;
mod test_canvas;

fn main() -> Result<(), eframe::Error> {
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
            let (circuit, canvas) = provide_circuit();
            Box::new(CirquilApp {circuit, canvas})
        }),
    )
}
