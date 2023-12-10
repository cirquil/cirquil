use std::cell::Cell;
use std::time::Instant;
use crate::core::circuit::Circuit;
use crate::core::component::Component;
use crate::core::components::{AndGate, ClockGenerator};
use crate::core::value::Value;
use crate::core::wire::Wire;

pub fn test_propagate() {
    let clock = ClockGenerator::create();

    let and_a = AndGate::from_bit_width(2);
    let and_b = AndGate::from_bit_width(2);

    and_a.set_pin_value(0, Value { mask: 0, value: 7 });
    and_a.set_pin_value(1, Value { mask: 0, value: 3 });

    and_b.set_pin_value(0, Value { mask: 0, value: 7 });
    and_b.set_pin_value(1, Value { mask: 0, value: 9 });

    let wire1 = Wire { value: Cell::new(Default::default()), connected_components: vec![(0, 2), (1, 0) ] };
    let wire2 = Wire { value: Cell::new(Default::default()), connected_components: vec![(2, 0), (1, 1) ] };

    and_a.set_pin_wire(2, Some(0));
    and_b.set_pin_wire(0, Some(0));

    and_b.set_pin_wire(1, Some(1));
    clock.set_pin_wire(0, Some(1));

    let circuit = Circuit {
        components: vec![ Box::new(and_a), Box::new(and_b), Box::new(clock) ],
        wires: vec![ wire1, wire2 ],
        clock_generators: vec![ 2 ],
    };

    println!("{:?} {:?}", circuit.components, circuit.wires);

    circuit.propagate();

    println!("{:?} {:?}", circuit.components, circuit.wires);

    circuit.propagate();

    println!("{:?} {:?}", circuit.components, circuit.wires);

    circuit.propagate();

    println!("{:?} {:?}", circuit.components, circuit.wires);

    let start = Instant::now();

    for _ in 0..1_000_000 {
        circuit.propagate();
    }

    println!("{:?} {:?} MHz", start.elapsed(), 1f64 / (start.elapsed().as_micros() as f64 / 1_000_000f64));
}