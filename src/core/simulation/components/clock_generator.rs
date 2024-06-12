use std::cell::Cell;
use std::fmt::{Debug, Formatter};

use serde::{Deserialize, Serialize};

use crate::core::canvas::location::Location;
use crate::core::simulation::component::{Behaviour, Component, ComponentModel, ComponentPins, ComponentProperties, Tick};
use crate::core::simulation::pin::{Direction, Pin};
use crate::core::simulation::value::Value;

#[derive(Clone, Serialize, Deserialize)]
pub struct ClockGenerator {
    pub(crate) value: Cell<u32>,
}

impl Behaviour for ClockGenerator {
    fn propagate(&self, pins: &ComponentPins, _properties: &ComponentProperties) {
        pins.set_value(0, Value::create(self.value.get(), 1));
    }
}

impl Debug for ClockGenerator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("ClockGenerator: {}", self.value.get()).as_str())
    }
}

impl ClockGenerator {
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
            component: ComponentModel::ClockGenerator(ClockGenerator {
                value: Cell::new(0),
            }),
        }
    }
}

impl Tick for ClockGenerator {
    fn tick(&self) {
        let old_value = self.value.get();

        let new_value = match old_value {
            0 => { 1 }
            1 => { 0 }
            _ => { 0 }
        };

        self.value.set(new_value);
    }
}