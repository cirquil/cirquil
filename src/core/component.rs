use std::fmt::Debug;
use crate::core::graphics::DrawContext;
use crate::core::pin::{Pin, PinIdx};
use crate::core::property::{Property, PropertyIdx};
use crate::core::value::Value;
use crate::core::wire::WireIdx;
use crate::gui::AsShapes;

pub type ComponentIdx = usize;

pub trait Behaviour {
    fn propagate(&self);
}

pub trait Appearance {
    fn draw(&self, ctx: Box<dyn DrawContext>);
}

pub trait Poke {
    fn mouse_pressed(&self) {}
    fn mouse_released(&self) {}
    fn mouse_dragged(&self) {}

    fn key_typed(&self) {}
}

pub trait OnTickStart {
    fn on_tick_start(&self) {}
}

pub trait Component: Behaviour + Appearance + Poke + OnTickStart + Debug + AsShapes {
    fn get_pins(&self) -> &Vec<Pin>;
    fn get_pin_value(&self, idx: PinIdx) -> Value;
    fn set_pin_value(&self, idx: PinIdx, value: Value);
    fn set_pin_wire(&self, pin: PinIdx, wire: Option<WireIdx>);

    fn get_properties(&self) -> &Vec<Box<dyn Property>>;
    fn get_property_value(&self, idx: PropertyIdx) -> String;
    fn set_property_value(&self, idx: PropertyIdx, value: String);
}
