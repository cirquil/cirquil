use std::cell::Cell;

use serde::{Deserialize, Serialize};

use crate::core::canvas::location::Location;
use crate::core::simulation::component::{Behaviour, Component, ComponentModel, ComponentPins, ComponentProperties};
use crate::core::simulation::pin::{Direction, Pin};
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
    pub fn create() -> Component {
        let pins = vec![
            Pin {
                value: Cell::new(Default::default()),
                bit_width: 1,
                direction: Direction::Output,
                wire: Cell::new(None),
                location: Location::new(0, 0),
            }
        ];

        Component {
            pins: ComponentPins::new(pins),
            properties: ComponentProperties::new(vec![]),
            component: ComponentModel::OutputPin(OutputPin {
                value: Default::default(),
            }),
        }
    }
}
