use std::cell::Cell;
use std::fmt::{Debug, Formatter};
use crate::core::component::{Appearance, Behaviour, OnTickStart, Poke};
use crate::core::graphics::DrawContext;
use crate::core::location::Location;
use crate::core::pin::{Direction, Pin};
use crate::core::value::Value;
use crate::declare_component;

declare_component! {
    pub struct ClockGenerator {
        value: Cell<u32>
    }
}

impl Behaviour for ClockGenerator {
    fn propagate(&self) {
        self.set_pin_value(0, Value::create(self.value.get(), 1));
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
        let s = pins.get(0).unwrap().value.get().get_defined_value();
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
                location: Location(0, 0)
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