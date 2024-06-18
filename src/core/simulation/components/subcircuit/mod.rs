use serde::{Deserialize, Serialize};

use crate::core::simulation::circuit::CircuitIdx;

pub mod input_pin;
pub mod output_pin;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Subcircuit {
    Instantiated(CircuitIdx),
    NotInstantiated(String),
}