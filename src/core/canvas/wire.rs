use serde::{Deserialize, Serialize};

use crate::core::canvas::location::Location;
use crate::core::simulation::wire::WireIdx;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasWire {
    pub wire: WireIdx,
    pub segments: Vec<(Location, Location)>,
    pub nodes: Vec<Location>,
}