use std::cell::Cell;

use serde::{Deserialize, Serialize};

use crate::core::canvas::location::Location;
use crate::core::simulation::component::{Behaviour, Component, ComponentModel, ComponentPins, ComponentProperties};
use crate::core::simulation::pin::{Direction, Pin};
use crate::core::simulation::property::{Property, StringProperty};
use crate::core::simulation::value::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputPin {
    pub value: Cell<Value>,
}

impl Behaviour for OutputPin {
    fn propagate(&self, pins: &ComponentPins, _properties: &ComponentProperties) {
        self.value.set(pins.get_value(0));
    }
}

impl OutputPin {
    pub fn create(label: &str) -> Component {
        let pins = vec![
            Pin {
                value: Cell::new(Default::default()),
                bit_width: 1,
                direction: Direction::Input,
                wire: Cell::new(None),
                location: Location::new(0, 0),
            }
        ];

        Component {
            pins: ComponentPins::new(pins),
            properties: ComponentProperties::new(vec![
                ("label".to_string(), Property::String(StringProperty::new(label.to_string())))
            ]),
            model: ComponentModel::OutputPin(OutputPin {
                value: Default::default(),
            }),
        }
    }
}
