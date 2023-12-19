use crate::core::component::ComponentIdx;
use crate::core::location::Location;
use crate::core::wire::WireIdx;

pub struct CanvasCircuit {
    pub components: Vec<CanvasComponent>,
    pub wires: Vec<CanvasWire>,
    pub circuit: usize,
    pub appearance: (),
    pub pins: (),
}

struct CanvasComponent {
    component: ComponentIdx,
    loc: Location,
}

struct CanvasWire {
    segments: Vec<(Location, Location)>,
    wire: WireIdx
}
