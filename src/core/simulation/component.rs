use std::fmt::Debug;
use crate::core::simulation::pin::{Pin, PinIdx};
use crate::core::simulation::property::{Property, PropertyIdx};
use crate::core::simulation::value::Value;
use crate::core::simulation::wire::WireIdx;
use crate::gui::component::{AsShapes, Poke};

pub type ComponentIdx = usize;

pub trait Behaviour {
    fn propagate(&self);
}

pub trait Tick {
    fn tick(&self) {}
}

pub trait Component: Behaviour + Tick + Poke + AsShapes + Debug {
    fn get_pins(&self) -> &Vec<Pin>;
    fn get_pin_value(&self, idx: PinIdx) -> Value;
    fn set_pin_value(&self, idx: PinIdx, value: Value);
    fn set_pin_wire(&self, pin: PinIdx, wire: Option<WireIdx>);

    fn get_properties(&self) -> &Vec<Box<dyn Property>>;
    fn get_property_value(&self, idx: PropertyIdx) -> String;
    fn set_property_value(&self, idx: PropertyIdx, value: String);
}
