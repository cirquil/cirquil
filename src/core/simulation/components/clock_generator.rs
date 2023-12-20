use std::cell::Cell;
use std::fmt::{Debug, Formatter};
use crate::core::simulation::component::{Behaviour, Tick};
use crate::core::canvas::location::Location;
use crate::core::simulation::pin::Direction;
use crate::declare_component;

declare_component! {
    pub struct ClockGenerator {
        value: Cell<u32>
    }
}

impl Behaviour for ClockGenerator {
    fn propagate(&self) {
        self.set_pin_value(0, Value::create(self.value.get(), 1));
    }
}

impl Debug for ClockGenerator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let pins = &self.pins;
        let s = pins.get(0).unwrap().value.get().get_defined_value();
        f.write_str(format!("ClockGenerator: {}", s).as_str())
    }
}

impl ClockGenerator {
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

        Self { pins, properties: vec![], value: Cell::new(0) }
    }
}

impl Tick for ClockGenerator {
    fn tick(&self) {
        let old_value = self.value.get();

        let new_value = match old_value {
            0 => { 1 },
            1 => { 0 },
            _ => { 0 }
        };

        self.value.set(new_value);
    }
}