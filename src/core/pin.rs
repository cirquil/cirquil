use std::cell::Cell;
use crate::core::location::Location;
use crate::core::value::Value;
use crate::core::wire::WireIdx;

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