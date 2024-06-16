use serde::{Deserialize, Serialize};
use crate::core::canvas::circuit::CanvasCircuit;
use crate::core::simulation::circuit::Circuit;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayFile {
    canvas_circuits: Vec<CanvasCircuit>,
    states: Vec<Circuit>,
}