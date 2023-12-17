use std::cell::Cell;
use std::time::Instant;
use crate::core::circuit::Circuit;
use crate::core::component::Component;
use crate::core::components::logic::and_gate::AndGate;
use crate::core::components::clock_generator::ClockGenerator;
use crate::core::components::logic::not_gate::NotGate;
use crate::core::components::logic::or_gate::OrGate;
use crate::core::value::Value;
use crate::core::wire::Wire;

pub fn test_propagate() {
    let clock = ClockGenerator::create();

    let and_a = AndGate::from_bit_width(2);
    let and_b = AndGate::from_bit_width(2);

    and_a.set_pin_value(0, Value::new(5, 0));
    and_a.set_pin_value(1, Value::new(3, 0));

    and_b.set_pin_value(0, Value::new(7, 0));
    and_b.set_pin_value(1, Value::new(9, 0));

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

pub fn test_or() {
    let clock = ClockGenerator::create();

    let or = OrGate::from_bit_width(2);

    or.set_pin_value(0, Value::new(8, 0));
    or.set_pin_value(1, Value::new(2, 0));

    let wire = Wire { value: Cell::new(Default::default()), connected_components: vec![(0, 0), (1, 0) ] };

    clock.set_pin_wire(0, Some(0));
    or.set_pin_wire(0, Some(0));

    let circuit = Circuit {
        components: vec![ Box::new(or), Box::new(clock) ],
        wires: vec![ wire ],
        clock_generators: vec![ 1 ],
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

pub fn test_not() {
    let clock = ClockGenerator::create();
    let not = NotGate::from_bit_width(2);

    let wire = Wire { value: Cell::new(Default::default()), connected_components: vec![(0, 0), (1, 0) ] };

    clock.set_pin_wire(0, Some(0));
    not.set_pin_wire(0, Some(0));

    let circuit = Circuit {
        components: vec![ Box::new(clock), Box::new(not) ],
        wires: vec![ wire ],
        clock_generators: vec![ 0 ],
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