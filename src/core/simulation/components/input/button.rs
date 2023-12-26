use std::cell::Cell;
use std::fmt::{Debug, Formatter};
use crate::core::simulation::component::{Behaviour, Tick};
use crate::core::canvas::location::Location;
use crate::core::simulation::pin::Direction;
use crate::declare_component;

declare_component! {
    pub struct InputButton {
        pub(crate) state: Cell<u32>
    }
}

impl Behaviour for InputButton {
    fn propagate(&self) {
        self.set_pin_value(0, Value::create(self.state.get(), 1));
    }
}

impl Debug for InputButton {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let pins = &self.pins;
        let s = pins.get(0).unwrap().value.get().get_defined_value();
        f.write_str(format!("InputButton: {}", s).as_str())
    }
}

impl InputButton {
    pub fn create() -> Self {
        let pins = vec![
            Pin {
                value: Cell::new(Default::default()),
                bit_width: 1,
                direction: Direction::OUTPUT,
                wire: Cell::new(None),
                location: Location::new(0, 0)
            }
        ];

        Self { pins, properties: vec![], state: Cell::new(0) }
    }
}

impl Tick for InputButton {}