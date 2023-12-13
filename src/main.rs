use crate::test_propagate::test_propagate;
use crate::test_converter::test_converter;

mod core;
mod gui;
mod logisim_converter;
mod test_propagate;
mod test_converter;

fn main() {
    test_propagate();
    test_converter();
}
