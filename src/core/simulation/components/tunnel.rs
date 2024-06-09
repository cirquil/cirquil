use std::cell::{Cell, RefCell};
use std::fmt::{Debug, Formatter};
use crate::core::simulation::component::{Behaviour, Tick};
use crate::core::canvas::location::Location;
use crate::core::simulation::pin::Direction;
use crate::core::simulation::property::{IntegerProperty, StringProperty};
use crate::declare_component;

declare_component! {
    pub struct Tunnel {
        pub(crate) name: String
    }
}

impl Debug for Tunnel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let pins = &self.pins;
        let a = pins.get(0).unwrap().value.get().get_defined_value();
        f.write_str(format!("Tunnel {}: {:?}", self.name, a).as_str())
    }
}

impl Behaviour for Tunnel {
    fn propagate(&self) {}
}

impl Tick for Tunnel {}

impl Tunnel {
    pub fn from_properties(properties: Vec<Box<dyn Property>>) -> Self {
        let bit_width = properties.get(0).unwrap().get_value().parse::<u8>().unwrap();
        let name = properties.get(1).unwrap().get_value();

        let pins = vec![
            Pin {
                value: Cell::new(Default::default()),
                bit_width,
                direction: Direction::OUTPUT,
                wire: Cell::new(None),
                location: Location::new(0, 0)
            }
        ];

        Self { pins, properties, name }
    }

    pub fn from_name_width(name: &str, bit_width: u8) -> Self {
        let properties: Vec<Box<dyn Property>> = vec![
            Box::new(IntegerProperty { name: "bit_width".to_string(), value: Cell::new(bit_width as u32) }),
            Box::new(StringProperty { name: "name".to_string(), value: RefCell::new(name.to_string()) })
        ];

        Self::from_properties(properties)
    }
}
