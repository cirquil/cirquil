use std::cell::Cell;
use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::core::canvas::location::Location;
use crate::core::simulation::component::{Behaviour, Component, ComponentModel, ComponentPins, ComponentProperties};
use crate::core::simulation::pin::{Direction, Pin};
use crate::core::simulation::property::{BoundedIntegerProperty, IntegerProperty, Property};
use crate::core::uuid::make_uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrGate;

impl Behaviour for OrGate {
    fn propagate(&self, pins: &ComponentPins, _properties: &ComponentProperties) {
        pins.set_value(
            2,
            pins.get_value(0) | pins.get_value(1),
        )
    }
}

impl OrGate {
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
                location: Location::new(-30, 10),
            },
            Pin {
                value: Cell::new(Default::default()),
                bit_width,
                direction: Direction::Input,
                wire: Cell::new(None),
                location: Location::new(-30, -10),
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
            model: ComponentModel::OrGate(OrGate),
            uuid: make_uuid(),
        }
    }

    pub fn from_bit_width(bit_width: u8) -> Component {
        let properties = ComponentProperties::new(vec![
            ("bit_width".to_string(), Property::Integer(IntegerProperty::new(bit_width as u32)))
        ]);

        Self::from_properties(properties)
    }
}
