use std::cell::Cell;

use serde::{Deserialize, Serialize};

use crate::core::canvas::location::Location;
use crate::core::simulation::component::{Component, ComponentModel, ComponentPins, ComponentProperties};
use crate::core::simulation::pin::{Direction, Pin};
use crate::core::simulation::property::{BoundedIntegerProperty, IntegerProperty, Property, StringProperty};
use crate::core::uuid::make_uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tunnel {
    pub(crate) name: String,
}

impl Tunnel {
    pub fn from_properties(properties: ComponentProperties) -> Component {
        let bit_width =
            properties
                .get("bit_width")
                .unwrap()
                .as_bounded_integer()
                .unwrap_or(&BoundedIntegerProperty::new(1, 10, 1))
                .get() as u8;

        let name =
            properties
                .get("name")
                .unwrap()
                .as_string()
                .unwrap_or(&StringProperty::new("Tunnel".to_string()))
                .get();

        let pins = vec![
            Pin {
                value: Cell::new(Default::default()),
                bit_width,
                direction: Direction::Input,
                wire: Cell::new(None),
                location: Location::new(0, 0),
            }
        ];

        Component {
            pins: ComponentPins::new(pins),
            properties,
            model: ComponentModel::Tunnel(Tunnel { name }),
            uuid: make_uuid(),
        }
    }

    pub fn from_name_width(name: &str, bit_width: u8) -> Component {
        let properties = ComponentProperties::new(vec![
            ("bit_width".to_string(), Property::Integer(IntegerProperty::new(bit_width as u32))),
            ("name".to_string(), Property::String(StringProperty::new(name.to_string()))),
        ]);

        Self::from_properties(properties)
    }
}
