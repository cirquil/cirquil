use std::cell::Cell;
use std::fmt::{Debug, Formatter};
use crate::core::component::{Appearance, Behaviour, OnTickStart, Poke};
use crate::core::graphics::DrawContext;
use crate::core::location::Location;
use crate::core::pin::{Direction, Pin};
use crate::core::property::{IntegerProperty};
use crate::core::value::Value;
use crate::declare_component;

declare_component! {
    pub struct AndGate {}
}

impl Debug for AndGate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let pins = &self.pins;
        let a = pins.get(0).unwrap().value.get().get_defined_value();
        let b = pins.get(1).unwrap().value.get().get_defined_value();
        let s = pins.get(2).unwrap().value.get().get_defined_value();
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
                location: Location(-30, 10)
            },
            Pin {
                value: Cell::new(Default::default()),
                bit_width,
                direction: Direction::INPUT,
                wire: Cell::new(None),
                location: Location(-30, -10)
            },
            Pin {
                value: Cell::new(Default::default()),
                bit_width,
                direction: Direction::OUTPUT,
                wire: Cell::new(None),
                location: Location(0, 0)
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