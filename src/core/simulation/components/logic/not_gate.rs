use std::cell::Cell;
use std::fmt::{Debug, Formatter};
use crate::core::simulation::component::{Behaviour, Tick};
use crate::core::canvas::location::Location;
use crate::core::simulation::pin::Direction;
use crate::core::simulation::property::IntegerProperty;
use crate::declare_component;

declare_component! {
    pub struct NotGate {}
}

impl Debug for NotGate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let pins = &self.pins;
        let a = pins.get(0).unwrap().value.get().get_defined_value();
        let s = pins.get(1).unwrap().value.get().get_defined_value();
        f.write_str(format!("NotGate: ~{:?} -> {:?}", a, s).as_str())
    }
}

impl Behaviour for NotGate {
    fn propagate(&self) {
        self.set_pin_value(
            1,
            !self.get_pin_value(0)
        );
    }
}

impl Tick for NotGate {}

impl NotGate {
    pub fn from_properties(properties: Vec<Box<dyn Property>>) -> Self {
        let bit_width = properties.get(0).unwrap().get_value().parse::<u8>().unwrap();

        let pins = vec![
            Pin {
                value: Cell::new(Default::default()),
                bit_width,
                direction: Direction::INPUT,
                wire: Cell::new(None),
                location: Location::new(-20, 0)
            },
            Pin {
                value: Cell::new(Default::default()),
                bit_width,
                direction: Direction::OUTPUT,
                wire: Cell::new(None),
                location: Location::new(0, 0)
            }
        ];

        Self { pins, properties }
    }

    pub fn from_bit_width(bit_width: u8) -> Self {
        let properties: Vec<Box<dyn Property>> = vec![
            Box::new(IntegerProperty { name: "bit_width".to_string(), value: Cell::new(bit_width as u32) })
        ];

        Self::from_properties(properties)
    }
}
