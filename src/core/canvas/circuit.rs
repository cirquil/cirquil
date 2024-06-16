use serde::{Deserialize, Serialize};

use crate::core::canvas::component::CanvasComponent;
use crate::core::canvas::wire::CanvasWire;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasCircuit {
    pub components: Vec<CanvasComponent>,
    pub wires: Vec<CanvasWire>,
    pub circuit: usize,
    pub appearance: (),
    pub pins: (),
}