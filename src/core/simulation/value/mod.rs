use std::fmt::{Debug, Formatter};
use std::ops::{BitAnd, BitOr, Not};

use serde::{Deserialize, Serialize};

use crate::core::simulation::value::operations::{and, not, or};

pub mod operations;

/*
union Value {
    uint64_t value;
    struct {
        uint32_t mask;
        uint32_t value;
    }
}
*/

#[derive(Debug)]
pub enum BitState {
    F,
    T,
    X,
    E,
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
#[repr(transparent)]
pub struct Value(u64);

impl Value {
    const BITS: u8 = 32;
}

impl Value {
    pub fn new(value: u32, mask: u32) -> Self {
        Value(
            ((mask as u64) << 32) + (value as u64)
        )
    }

    /// Creates new [`Value`](Value) with given bit width
    pub fn create(value: u32, bits: u8) -> Self {
        Self::new(
            value & !(u32::MAX << bits),
            u32::MAX << bits,
        )
    }
}

impl Value {
    // #[inline(always)]
    pub fn get_raw_mask(&self) -> u32 {
        (self.0 >> 32) as u32
    }

    pub fn get_raw_value(&self) -> u32 {
        self.0 as u32
    }

    pub fn get_defined_value(&self) -> u32 {
        self.get_raw_value() & !self.get_raw_mask()
    }

    pub fn get_value_pull_up(&self) -> u32 {
        self.get_raw_value() | self.get_raw_mask()
    }

    pub fn get_value_pull_down(&self) -> u32 {
        self.get_raw_value() & !self.get_raw_mask()
    }

    pub fn get_undefined(&self) -> u32 {
        self.get_raw_mask() & !self.get_raw_value()
    }

    pub fn get_error(&self) -> u32 {
        self.get_raw_mask() & self.get_raw_value()
    }

    pub fn is_fully_defined(&self) -> bool {
        self.get_undefined().count_ones() != 0
    }

    pub fn is_error(&self) -> bool {
        self.get_error().count_ones() != 0
    }
}

impl Value {
    #[inline(always)]
    fn set_raw_bit(&self, pos: u8) -> Self {
        Self(self.0 | (1 << pos))
    }

    #[inline(always)]
    fn clear_raw_bit(&self, pos: u8) -> Self {
        Self(self.0 & !(1 << pos))
    }

    #[inline(always)]
    fn get_raw_bit(&self, pos: u8) -> bool {
        ((self.0 >> pos) & 1) != 0
    }
}

impl Value {
    fn get_bit_state(&self, pos: u8) -> BitState {
        match (self.get_raw_bit(pos + 32), self.get_raw_bit(pos)) {
            (false, false) => { BitState::F }
            (false, true) => { BitState::T }
            (true, false) => { BitState::X }
            (true, true) => { BitState::E }
        }
    }

    fn set_bit_state(&self, pos: u8, state: BitState) -> Self {
        match state {
            BitState::F => { self.clear_raw_bit(pos + 32).clear_raw_bit(pos) }
            BitState::T => { self.clear_raw_bit(pos + 32).set_raw_bit(pos) }
            BitState::X => { self.set_raw_bit(pos + 32).clear_raw_bit(pos) }
            BitState::E => { self.set_raw_bit(pos + 32).set_raw_bit(pos) }
        }
    }
}

impl Value {
    pub fn apply_unary(&self, function: fn(BitState) -> BitState) -> Self {
        self.apply_unary_range(function, 0, Self::BITS)
    }

    pub fn apply_unary_range(&self, function: fn(BitState) -> BitState, from: u8, to: u8) -> Self {
        let mut result = Value(0);

        for i in from..to {
            result = result.set_bit_state(
                i,
                function(self.get_bit_state(i)),
            );
        }

        result
    }

    pub fn apply_binary(&self, rhs: Self, function: fn(BitState, BitState) -> BitState) -> Self {
        self.apply_binary_range(rhs, function, 0, Self::BITS)
    }

    pub fn apply_binary_range(
        &self, rhs: Self,
        function: fn(BitState, BitState) -> BitState,
        from: u8, to: u8) -> Self {
        let mut result = Value(0);

        for i in from..to {
            result = result.set_bit_state(
                i,
                function(self.get_bit_state(i), rhs.get_bit_state(i)),
            );
        }

        result
    }

    // let mut i = 0;
    // while i < Self::BITS {
    //     result = result.set_bit_state(
    //         i,
    //         function(self.get_bit_state(i), rhs.get_bit_state(i))
    //     );
    //     i += 1;
    // }

    // self.set_bit_state(
    //     0,
    //     function(self.get_bit_state(0), rhs.get_bit_state(0))
    // )
}

impl Default for Value {
    fn default() -> Self {
        Value::new(0, u32::MAX)
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..Self::BITS {
            let _ = self.get_bit_state(i).fmt(f);
        }

        Ok(())
    }
}

impl BitAnd for Value {
    type Output = Value;

    fn bitand(self, rhs: Self) -> Self::Output {
        // Value::new(self.get_raw_value() & rhs.get_raw_value(), 0)
        self.apply_binary(rhs, and)
    }
}

impl BitOr for Value {
    type Output = Value;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.apply_binary(rhs, or)
    }
}

impl Not for Value {
    type Output = Value;

    fn not(self) -> Self::Output {
        self.apply_unary(not)
    }
}

// #[derive(Debug, Copy, Clone)]
// pub struct Value {
//     pub mask: u32,
//     pub value: u32
// }

// impl Default for Value {
//     fn default() -> Self {
//         Value {
//             mask: 0xff_ff_ff_ff,
//             value: 0
//         }
//     }
// }

// impl BitAnd for Value {
//     type Output = Value;
//
//     fn bitand(self, rhs: Self) -> Self::Output {
//         Value {
//             mask: self.mask & rhs.mask,
//             value: self.value & rhs.value,
//         }
//     }
// }

// #[repr(C)]
// union Value {
//     value: u64,
//     foo: CompositeValue
// }
