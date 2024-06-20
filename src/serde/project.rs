use std::collections::HashMap;
use std::io::Error;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::core::canvas::location::Location;
use crate::core::simulation::component::Component;
use crate::core::simulation::pin::Direction;
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

pub fn save_project<P>(project_file: &ProjectFile, path: P) -> Result<(), Error>
    where
        P: AsRef<Path>,
{
    serialize_to_file(project_file, path)
}

pub fn load_project<P>(path: P) -> Result<ProjectFile, Error>
    where
        P: AsRef<Path>,
{
    deserialize_from_file(path)
}
