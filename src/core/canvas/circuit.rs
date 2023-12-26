use crate::core::canvas::component::CanvasComponent;
use crate::core::canvas::wire::CanvasWire;

pub struct CanvasCircuit {
    pub components: Vec<CanvasComponent>,
    pub wires: Vec<CanvasWire>,
    pub circuit: usize,
    pub appearance: (),
    pub pins: (),
}