use std::collections::HashMap;
use std::io::Error;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::core::canvas::location::Location;
use crate::core::simulation::component::Component;
use crate::core::simulation::pin::{Direction, Pin};
use crate::serde::fs::{deserialize_from_file, serialize_to_file};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedComponent {
    pub location: Location,
    pub component: Component,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedWire {
    pub start: Location,
    pub end: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedCircuitBounds {
    pub start: Location,
    pub end: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedCircuitPin {
    pub location: Location,
    pub label: String,
    pub bit_width: u8,
    pub direction: Direction,
}

impl From<SavedCircuitPin> for Pin {
    fn from(value: SavedCircuitPin) -> Self {
        Self {
            value: Default::default(),
            bit_width: value.bit_width,
            direction: value.direction,
            wire: Default::default(),
            location: value.location,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedCircuit {
    pub components: Vec<SavedComponent>,
    pub wires: Vec<SavedWire>,
    pub bounds: SavedCircuitBounds,
    pub pins: Vec<SavedCircuitPin>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectFile {
    pub top_circuit: String,
    pub circuits: HashMap<String, SavedCircuit>,
}

impl ProjectFile {
    pub fn save<P>(&self, path: P) -> Result<(), Error>
        where
            P: AsRef<Path>,
    {
        serialize_to_file(self, path)
    }

    pub fn load<P>(path: P) -> Result<Self, Error>
        where
            P: AsRef<Path>,
    {
        deserialize_from_file(path)
    }
}
