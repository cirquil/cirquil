use serde::{Deserialize, Serialize};

use crate::core::canvas::component::CanvasComponent;
use crate::core::canvas::wire::CanvasWire;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CanvasCircuit {
    pub name: String,
    pub components: Vec<CanvasComponent>,
    pub wires: Vec<CanvasWire>,
    pub appearance: (),
    pub pins: (),
}