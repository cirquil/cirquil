use std::cell::Cell;

use serde::{Deserialize, Serialize};

use crate::core::canvas::location::Location;
use crate::core::simulation::component::{Behaviour, Component, ComponentModel, ComponentPins, ComponentProperties};
use crate::core::simulation::pin::{Direction, Pin};
use crate::core::simulation::property::{BoundedIntegerProperty, IntegerProperty, Property};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotGate;

impl Behaviour for NotGate {
    fn propagate(&self, pins: &ComponentPins, _properties: &ComponentProperties) {
        pins.set_value(
            1,
            !pins.get_value(0),
        )
    }
}

impl NotGate {
    pub fn from_properties(properties: ComponentProperties) -> Component {
        let bit_width =
            properties
                .get("bit_width")
                .unwrap()
                .as_bounded_integer()
                .unwrap_or(&BoundedIntegerProperty::new(1, 10, 1))
                .get() as u8;

        let pins = vec![
            Pin {
                value: Cell::new(Default::default()),
                bit_width,
                direction: Direction::Input,
                wire: Cell::new(None),
                location: Location::new(-20, 0),
            },
            Pin {
                value: Cell::new(Default::default()),
                bit_width,
                direction: Direction::Output,
                wire: Cell::new(None),
                location: Location::new(0, 0),
            },
        ];

        Component {
            pins: ComponentPins::new(pins),
            properties,
            component: ComponentModel::NotGate(NotGate),
        }
    }

    pub fn from_bit_width(bit_width: u8) -> Component {
        let properties = ComponentProperties::new(vec![
            ("bit_width".to_string(), Property::Integer(IntegerProperty::new(bit_width as u32)))
        ]);

        Self::from_properties(properties)
    }
}
