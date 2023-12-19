use crate::test_canvas::test_canvas;
use crate::test_propagate::{test_not, test_or, test_propagate};

mod core;
mod gui;
mod logisim;
mod test_propagate;
mod test_canvas;

fn main() {
    // test_not();
    // test_propagate();
    // test_or();

    test_canvas();
}
