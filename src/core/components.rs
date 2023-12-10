use std::cell::Cell;
use std::fmt::{Debug, Formatter};
use crate::core::component::{Appearance, Behaviour, OnTickStart, Component, Poke};
use crate::core::graphics::DrawContext;
use crate::core::pin::{Direction, Pin, PinIdx};
use crate::core::property::{IntegerProperty, Property};
use crate::core::value::Value;
use crate::core::wire::WireIdx;


macro_rules! declare_component {
    (
        $(#[derive($($derive:meta),*)])?
        $pub:vis struct $name:ident {
            $($fpub:vis $field:ident : $type:ty),*
        }
    ) => {
        $(#[derive($($derive),*)])?
        $pub struct $name {
            pins: Vec<Pin>,
            properties: Vec<Box<dyn Property>>,
            $($fpub $field : $type),*
        }
        impl $name {
            $pub fn new(
                pins: Vec<Pin>,
                properties: Vec<Box<dyn Property>>,
                $($field:$type,)*)
            -> Self {
                Self {
                    pins, properties,
                    $($field,)*
                }
            }
        }

        impl Component for $name {
            fn get_pins(&self) -> &Vec<Pin> {
                &self.pins
            }

            fn get_pin_value(&self, idx: usize) -> Value {
                self.pins[idx].value.get()
            }

            fn set_pin_value(&self, idx: usize, value: Value) {
                self.pins[idx].value.set(value);
            }

            fn set_pin_wire(&self, pin: PinIdx, wire: Option<WireIdx>) {
                self.pins.get(pin).unwrap().wire.set(wire);
            }

            fn get_properties(&self) -> &Vec<Box<dyn Property>> {
                &self.properties
            }

            fn get_property_value(&self, idx: usize) -> String {
                self.properties[idx].get_value()
            }

            fn set_property_value(&self, idx: usize, value: String) {
                self.properties[idx].parse(value.as_str()).unwrap()
            }
        }
    }
}

declare_component! {
    pub struct AndGate {}
}

impl Debug for AndGate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let pins = &self.pins;
        let a = pins.get(0).unwrap().value.get().value;
        let b = pins.get(1).unwrap().value.get().value;
        let s = pins.get(2).unwrap().value.get().value;
        f.write_str(format!("AndGate: {} & {} -> {}", a, b, s).as_str())
    }
}

impl Behaviour for AndGate {
    fn propagate(&self) {
        self.set_pin_value(
            2,
            self.get_pin_value(0) & self.get_pin_value(1)
        );
    }
}

impl Appearance for AndGate {
    fn draw(&self, ctx: Box<dyn DrawContext>) {
        ctx.draw_line();
    }
}

impl Poke for AndGate {}

impl OnTickStart for AndGate {}

impl AndGate {
    pub fn from_properties(properties: Vec<Box<dyn Property>>) -> Self {
        let bit_width = properties.get(0).unwrap().get_value().parse::<u8>().unwrap();

        let pins = vec![
            Pin {
                value: Cell::new(Default::default()),
                bit_width,
                direction: Direction::INPUT,
                wire: Cell::new(None),
            },
            Pin {
                value: Cell::new(Default::default()),
                bit_width,
                direction: Direction::INPUT,
                wire: Cell::new(None),
            },
            Pin {
                value: Cell::new(Default::default()),
                bit_width,
                direction: Direction::OUTPUT,
                wire: Cell::new(None),
            }
        ];

        Self { pins, properties }
    }

    pub fn from_bit_width(bit_width: u8) -> Self {
        let properties: Vec<Box<dyn Property>> = vec![
            Box::new(IntegerProperty { name: "bit_width".to_string(), value: Cell::new(bit_width as u32) })
        ];

        Self::from_properties(properties)
    }
}

declare_component! {
    pub struct ClockGenerator {
        value: Cell<u32>
    }
}

impl Behaviour for ClockGenerator {
    fn propagate(&self) {
        self.set_pin_value(0, Value { mask: 0, value: self.value.get() });
    }
}

impl Appearance for ClockGenerator {
    fn draw(&self, ctx: Box<dyn DrawContext>) {
        ctx.draw_line();
    }
}

impl Debug for ClockGenerator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let pins = &self.pins;
        let s = pins.get(0).unwrap().value.get().value;
        f.write_str(format!("ClockGenerator: {}", s).as_str())
    }
}

impl Poke for ClockGenerator {}

impl ClockGenerator {
    pub fn create() -> Self {
        let pins = vec![
            Pin {
                value: Cell::new(Default::default()),
                bit_width: 1,
                direction: Direction::OUTPUT,
                wire: Cell::new(None),
            }
        ];

        Self { pins, properties: vec![], value: Cell::new(0) }
    }
}

impl OnTickStart for ClockGenerator {
    fn on_tick_start(&self) {
        let old_value = self.value.get();

        let new_value = match old_value {
            0 => { 1 },
            1 => { 0 },
            _ => { 0 }
        };

        self.value.set(new_value);
    }
}

// pub struct AndGate {
//     pins: Vec<Pin>,
//     properties: Vec<Box<dyn Property>>
// }
//
// impl Behaviour for AndGate {
//     fn propagate(&self) {
//         self.set_pin_value(
//             2,
//             self.get_pin_value(0) & self.get_pin_value(1)
//         );
//     }
// }
//
// impl Appearance for AndGate {
//     fn draw(&self, ctx: Box<dyn DrawContext>) {
//         ctx.draw_line();
//     }
// }
//
// impl Poke for AndGate {}
//
// impl Component for AndGate {
//     fn get_pin_value(&self, idx: usize) -> Value {
//         self.pins[idx].value.get()
//     }
//
//     fn set_pin_value(&self, idx: usize, value: Value) {
//         self.pins[idx].value.set(value);
//     }
//
//     fn get_property_value(&self, idx: usize) -> String {
//         self.properties[idx].get_value()
//     }
//
//     fn set_property_value(&self, idx: usize, value: String) {
//         self.properties[idx].parse(value.as_str()).unwrap()
//     }
// }
//
// impl AndGate {
//     fn new(pins: Vec<Pin>, properties: Vec<Box<dyn Property>>) -> Self {
//         Self { pins, properties }
//     }
// }