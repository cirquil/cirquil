use std::cell::{Cell, RefCell};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Property {
    Integer(IntegerProperty),
    BoundedInteger(BoundedIntegerProperty),
    String(StringProperty),
}

impl Property {
    pub fn as_integer(&self) -> Option<&IntegerProperty> {
        if let Property::Integer(p) = self { Some(p) } else { None }
    }

    pub fn as_bounded_integer(&self) -> Option<&BoundedIntegerProperty> {
        if let Property::BoundedInteger(p) = self { Some(p) } else { None }
    }

    pub fn as_string(&self) -> Option<&StringProperty> {
        if let Property::String(p) = self { Some(p) } else { None }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericProperty<T>(T);

pub type CellProperty<V> = GenericProperty<Cell<V>>;

impl<V: Copy> CellProperty<V> {
    pub fn get(&self) -> V {
        self.0.get()
    }
}

impl<V> CellProperty<V> {
    pub fn set(&self, value: V) {
        self.0.set(value)
    }

    pub fn new(value: V) -> Self {
        CellProperty {
            0: Cell::new(value),
        }
    }
}

pub type RefCellProperty<V> = GenericProperty<RefCell<V>>;

impl<V: Clone> RefCellProperty<V> {
    pub fn get(&self) -> V {
        self.0.borrow().clone()
    }
}

impl<V> RefCellProperty<V> {
    pub fn set(&self, value: V) {
        self.0.replace(value);
    }

    pub fn new(value: V) -> Self {
        RefCellProperty {
            0: RefCell::new(value)
        }
    }
}

pub type IntegerProperty = CellProperty<u32>;
pub type StringProperty = RefCellProperty<String>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundedIntegerProperty {
    min: u32,
    max: u32,
    value: Cell<u32>,
}

#[derive(Debug)]
pub struct BoundsError {
    min: u32,
    max: u32,
    value: u32,
}

impl Display for BoundsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!("Value {} is out of bounds: [{}; {})",
                    self.value, self.min, self.max)
                .as_str()
        )
    }
}

impl Error for BoundsError {}

impl BoundedIntegerProperty {
    pub fn get(&self) -> u32 {
        self.value.get()
    }

    pub fn set(&self, value: u32) -> Result<(), BoundsError> {
        if (self.min <= value) && (value < self.max) {
            self.value.set(value);
            Ok(())
        } else {
            Err(BoundsError {
                min: self.min,
                max: self.max,
                value,
            })
        }
    }

    pub fn new(min: u32, max: u32, value: u32) -> Self {
        BoundedIntegerProperty {
            min,
            max,
            value: Cell::new(value),
        }
    }
}
