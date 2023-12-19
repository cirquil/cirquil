use std::cell::Cell;
use crate::core::canvas::{CanvasCircuit, CanvasComponent, CanvasWire};
use crate::core::circuit::Circuit;
use crate::core::component::Component;
use crate::core::components::clock_generator::ClockGenerator;
use crate::core::components::logic::or_gate::OrGate;
use crate::core::location::Location;
use crate::core::wire::Wire;

pub fn test_canvas() {
    // Model
    let or = OrGate::from_bit_width(1);
    let clock = ClockGenerator::create();

    let wire = Wire { value: Cell::new(Default::default()), connected_components: vec![
        (0, 0), (1, 0)
    ] };

    let display_wire = Wire { value: Cell::new(Default::default()), connected_components: vec![
        (1, 2)
    ] };

    clock.set_pin_wire(0, Some(0));

    or.set_pin_wire(0, Some(0));
    or.set_pin_wire(2, Some(1));

    let circuit = Circuit {
        components: vec![Box::new(clock), Box::new(or)],
        wires: vec![wire, display_wire],
        clock_generators: vec![0],
    };

    // View
    let canvasOr = CanvasComponent { component: 1, loc: Location(60, 20) };
    let canvasClock = CanvasComponent { component: 0, loc: Location(20, 20) };

    let canvasWire = CanvasWire { segments: vec![(Location(20, 20), Location(60, 20))], wire: 0 };
    let canvasDisplayWire = CanvasWire { segments: vec![
        (Location(80, 30), Location(100, 30)),
        (Location(100, 30), Location(100, 60))
    ], wire: 1 };

    let canvasCircuit = CanvasCircuit {
        components: vec![canvasClock, canvasOr],
        wires: vec![canvasWire, canvasDisplayWire],
        circuit: 0,
        appearance: (),
        pins: (),
    };

    println!("{:?} {:?}", circuit.components, circuit.wires);

    circuit.propagate();

    println!("{:?} {:?}", circuit.components, circuit.wires);
}