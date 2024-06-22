use serde::{Deserialize, Serialize};

use crate::core::simulation::circuit::CircuitIdx;
use crate::core::simulation::wire::WireIdx;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Probe {
    pub name: String,
    pub circuit: CircuitIdx,
    pub wire: WireIdx,
}
