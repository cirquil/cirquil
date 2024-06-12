use std::cell::Cell;
use std::fmt::{Debug, Formatter};

use serde::{Deserialize, Serialize};

use crate::core::canvas::location::Location;
use crate::core::simulation::component::{Behaviour, Component, ComponentModel, ComponentPins, ComponentProperties};
use crate::core::simulation::pin::{Direction, Pin};
use crate::core::simulation::value::Value;

#[derive(Clone, Serialize, Deserialize)]
pub struct InputButton {
    pub(crate) state: Cell<u32>,
}

impl Behaviour for InputButton {
    fn propagate(&self, pins: &ComponentPins, properties: &ComponentProperties) {
        pins.set_value(0, Value::create(self.state.get(), 1));
    }
}

impl Debug for InputButton {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("InputButton: {}", self.state.get()).as_str())
    }
}

impl InputButton {
    pub fn create() -> Component {
        let pins = vec![
            Pin {
                value: Cell::new(Default::default()),
                bit_width: 1,
                direction: Direction::OUTPUT,
                wire: Cell::new(None),
                location: Location::new(0, 0),
            }
        ];

        Component {
            pins: ComponentPins::new(pins),
            properties: ComponentProperties::new(vec![]),
            component: ComponentModel::InputButton(InputButton {
                state: Cell::new(0),
            }),
        }
    }
}
