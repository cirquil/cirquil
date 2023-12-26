use std::cell::Cell;
use crate::core::canvas::location::Location;
use crate::core::simulation::value::Value;
use crate::core::simulation::wire::WireIdx;

pub type PinIdx = usize;

#[derive(Debug, PartialEq)]
pub enum Direction {
    INPUT,
    OUTPUT,
    INOUT
}

#[derive(Debug)]
pub struct Pin {
    pub value: Cell<Value>,
    pub bit_width: u8,
    pub direction: Direction,
    pub wire: Cell<Option<WireIdx>>,
    pub location: Location
}