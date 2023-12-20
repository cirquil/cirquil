use std::cell::Cell;
use crate::core::circuit::Circuit;
use crate::core::component::Component;
use crate::core::components::clock_generator::ClockGenerator;
use crate::core::components::logic::or_gate::OrGate;
use crate::core::wire::Wire;
use crate::logisim::converter::{convert_circuit, parse_logisim};

pub fn test_canvas() {
    let parsed = parse_logisim("test.circ");
    let (circuit, _) = convert_circuit(parsed, 0);

    // Model
    let or = OrGate::from_bit_width(1);
    let clock = ClockGenerator::create();

    let wire = Wire {
        value: Cell::new(Default::default()),
        connected_components: vec![
            (0, 0), (1, 0),
        ],
    };

    let display_wire = Wire {
        value: Cell::new(Default::default()),
        connected_components: vec![
            (1, 2)
        ],
    };

    clock.set_pin_wire(0, Some(0));

    or.set_pin_wire(0, Some(0));
    or.set_pin_wire(2, Some(1));


    println!("Hardcoded");
    let circuit_hc = Circuit {
        components: vec![Box::new(clock), Box::new(or)],
        wires: vec![wire, display_wire],
        clock_generators: vec![0],
    };
    println!("{:?} {:?}", circuit_hc.components, circuit_hc.wires);
    circuit_hc.propagate();
    println!("{:?} {:?}", circuit_hc.components, circuit_hc.wires);

    println!("From file");
    println!("{:?} {:?}", circuit.components, circuit.wires);
    circuit.propagate();
    println!("{:?} {:?}", circuit.components, circuit.wires);
    circuit.propagate();
    println!("{:?} {:?}", circuit.components, circuit.wires);
    circuit.propagate();
    println!("{:?} {:?}", circuit.components, circuit.wires);
    circuit.propagate();
    println!("{:?} {:?}", circuit.components, circuit.wires);
    circuit.propagate();
    println!("{:?} {:?}", circuit.components, circuit.wires);
    circuit.propagate();
    println!("{:?} {:?}", circuit.components, circuit.wires);
    circuit.propagate();
    println!("{:?} {:?}", circuit.components, circuit.wires);
    circuit.propagate();
    println!("{:?} {:?}", circuit.components, circuit.wires);
}