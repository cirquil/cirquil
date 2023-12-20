use std::cell::Cell;
use std::fmt::{Debug, Formatter};
use crate::core::simulation::component::{Behaviour, Tick};
use crate::core::canvas::location::Location;
use crate::core::simulation::pin::Direction;
use crate::core::simulation::property::IntegerProperty;
use crate::declare_component;

declare_component! {
    pub struct OrGate {}
}

impl Debug for OrGate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let pins = &self.pins;
        let a = pins.get(0).unwrap().value.get().get_defined_value();
        let b = pins.get(1).unwrap().value.get().get_defined_value();
        let s = pins.get(2).unwrap().value.get().get_defined_value();
        // let a = pins.get(0).unwrap().value.get();
        // let b = pins.get(1).unwrap().value.get();
        // let s = pins.get(2).unwrap().value.get();
        f.write_str(format!("OrGate: {:?} | {:?} -> {:?}", a, b, s).as_str())
    }
}

impl Behaviour for OrGate {
    fn propagate(&self) {
        let a = self.get_pin_value(0);
        let b = self.get_pin_value(1);
        let c = a | b;
        self.set_pin_value(
            2,
            c
        );
    }
}

impl Tick for OrGate {}

impl OrGate {
    pub fn from_properties(properties: Vec<Box<dyn Property>>) -> Self {
        let bit_width = properties.get(0).unwrap().get_value().parse::<u8>().unwrap();

        let pins = vec![
            Pin {
                value: Cell::new(Default::default()),
                bit_width,
                direction: Direction::INPUT,
                wire: Cell::new(None),
                location: Location::new(-30, 10)
            },
            Pin {
                value: Cell::new(Default::default()),
                bit_width,
                direction: Direction::INPUT,
                wire: Cell::new(None),
                location: Location::new(-30, -10)
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
