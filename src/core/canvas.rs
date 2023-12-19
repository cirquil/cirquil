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

pub struct CanvasComponent {
    pub component: ComponentIdx,
    pub loc: Location,
}

pub struct CanvasWire {
    pub segments: Vec<(Location, Location)>,
    pub wire: WireIdx
}
