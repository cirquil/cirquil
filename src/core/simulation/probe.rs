use crate::core::canvas::location::Location;
use crate::core::simulation::circuit::CircuitIdx;
use crate::core::simulation::wire::WireIdx;

#[derive(Debug, Clone)]
pub struct CanvasProbe {
    pub location: Location,
    pub probe: Probe,
}

#[derive(Debug, Clone)]
pub struct Probe {
    pub name: String,
    pub circuit: CircuitIdx,
    pub wire: WireIdx,
}
