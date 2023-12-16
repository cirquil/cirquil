use crate::test_propagate::{test_or, test_propagate};

mod core;
mod logisim;
mod gui;
mod test_propagate;

fn main() {
    test_propagate();
    test_or();
}
