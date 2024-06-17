use serde::{Deserialize, Serialize};
use crate::core::simulation::circuit::CircuitIdx;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Subcircuit {
    Instantiated(CircuitIdx),
    NotInstantiated(String),
}
