use std::collections::HashMap;
use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::simulation::components::clock_generator::ClockGenerator;
use crate::core::simulation::components::input::button::InputButton;
use crate::core::simulation::components::logic::and_gate::AndGate;
use crate::core::simulation::components::logic::not_gate::NotGate;
use crate::core::simulation::components::logic::or_gate::OrGate;
use crate::core::simulation::components::subcircuit::input_pin::InputPin;
use crate::core::simulation::components::subcircuit::output_pin::OutputPin;
use crate::core::simulation::components::subcircuit::Subcircuit;
use crate::core::simulation::components::tunnel::Tunnel;
use crate::core::simulation::pin::{Pin, PinIdx};
use crate::core::simulation::property::Property;
use crate::core::simulation::value::Value;
use crate::core::simulation::wire::WireIdx;

pub type ComponentIdx = usize;

pub trait Behaviour {
    fn propagate(&self, pins: &ComponentPins, properties: &ComponentProperties);
}

pub trait Tick {
    fn tick(&self);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    pub pins: ComponentPins,
    pub properties: ComponentProperties,
    pub model: ComponentModel,
    pub uuid: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComponentModel {
    ClockGenerator(ClockGenerator),
    AndGate(AndGate),
    OrGate(OrGate),
    NotGate(NotGate),
    InputButton(InputButton),
    Tunnel(Tunnel),

    InputPin(InputPin),
    OutputPin(OutputPin),
    Subcircuit(Subcircuit),
}

impl Component {
    pub fn get_pins(&self) -> &[Pin] { self.pins.get_pins() }
    pub fn get_pin_value(&self, idx: PinIdx) -> Value { self.pins.get_value(idx) }
    pub fn set_pin_value(&self, idx: PinIdx, value: Value) { self.pins.set_value(idx, value) }
    pub fn set_pin_wire(&self, pin: PinIdx, wire: Option<WireIdx>) {
        self.pins.get_pins().get(pin).unwrap().wire.set(wire)
    }

    pub fn get_properties(&self) -> &ComponentProperties { &self.properties }
    pub fn get_property(&self, name: &str) -> &Property {
        self.properties.get(name).unwrap()
    }

    pub fn propagate(&self) {
        match &self.model {
            ComponentModel::ClockGenerator(c) => { c.propagate(&self.pins, &self.properties) }
            ComponentModel::AndGate(c) => { c.propagate(&self.pins, &self.properties) }
            ComponentModel::OrGate(c) => { c.propagate(&self.pins, &self.properties) }
            ComponentModel::NotGate(c) => { c.propagate(&self.pins, &self.properties) }
            ComponentModel::InputButton(c) => { c.propagate(&self.pins, &self.properties) }
            ComponentModel::Tunnel(_) => {}

            ComponentModel::InputPin(c) => { c.propagate(&self.pins, &self.properties) }
            ComponentModel::OutputPin(c) => { c.propagate(&self.pins, &self.properties) }
            ComponentModel::Subcircuit(c) => { c.propagate(&self.pins, &self.properties) }
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ComponentProperties(HashMap<String, Property>);

impl ComponentProperties {
    pub fn get(&self, name: &str) -> Option<&Property> {
        self.0.get(name)
    }

    pub fn new(properties: Vec<(String, Property)>) -> Self {
        let properties_map: HashMap<String, Property> = properties.into_iter().collect();

        ComponentProperties(properties_map)
    }

    // pub fn new(properties: HashMap<String, Property>) -> Self {
    //     ComponentProperties {
    //         0: properties,
    //     }
    // }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ComponentPins(Vec<Pin>);

impl ComponentPins {
    pub fn get_pins(&self) -> &[Pin] {
        self.0.as_slice()
    }

    pub fn set_value(&self, pin_number: PinIdx, value: Value) {
        self.0.get(pin_number).unwrap().value.set(value);
    }

    pub fn get_value(&self, pin_number: PinIdx) -> Value {
        self.0.get(pin_number).unwrap().value.get()
    }
    pub fn new(pins: Vec<Pin>) -> Self {
        ComponentPins(pins)
    }
}
