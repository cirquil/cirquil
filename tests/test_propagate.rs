use std::cell::Cell;
use std::time::Instant;

use cirquil::core::simulation::circuit::Circuit;
use cirquil::core::simulation::components::clock_generator::ClockGenerator;
use cirquil::core::simulation::components::logic::and_gate::AndGate;
use cirquil::core::simulation::components::logic::not_gate::NotGate;
use cirquil::core::simulation::components::logic::or_gate::OrGate;
use cirquil::core::simulation::value::Value;
use cirquil::core::simulation::wire::Wire;

#[test]
pub fn test_propagate() {
    let clock = ClockGenerator::create();

    let and_a = AndGate::from_bit_width(2);
    let and_b = AndGate::from_bit_width(2);

    and_a.set_pin_value(0, Value::new(5, 0));
    and_a.set_pin_value(1, Value::new(3, 0));

    and_b.set_pin_value(0, Value::new(7, 0));
    and_b.set_pin_value(1, Value::new(9, 0));

    let wire1 = Wire { value: Cell::new(Default::default()), connected_components: vec![(0, 2), (1, 0)] };
    let wire2 = Wire { value: Cell::new(Default::default()), connected_components: vec![(2, 0), (1, 1)] };

    and_a.set_pin_wire(2, Some(0));
    and_b.set_pin_wire(0, Some(0));

    and_b.set_pin_wire(1, Some(1));
    clock.set_pin_wire(0, Some(1));

    let circuit = Circuit {
        components: vec![and_a, and_b, clock],
        wires: vec![wire1, wire2],
        clock_generators: vec![2],
        input_pins: vec![],
        output_pins: vec![],
    };

    println!("{:?} {:?}", circuit.components, circuit.wires);

    circuit.propagate_all();

    println!("{:?} {:?}", circuit.components, circuit.wires);

    circuit.propagate_all();

    println!("{:?} {:?}", circuit.components, circuit.wires);

    circuit.propagate_all();

    println!("{:?} {:?}", circuit.components, circuit.wires);

    let start = Instant::now();

    for _ in 0..1_000_000 {
        circuit.propagate_all();
    }

    println!("{:?} {:?} MHz", start.elapsed(), 1f64 / (start.elapsed().as_micros() as f64 / 1_000_000f64));
}

#[test]
pub fn test_or() {
    let clock = ClockGenerator::create();

    let or = OrGate::from_bit_width(2);

    or.set_pin_value(0, Value::new(8, 0));
    or.set_pin_value(1, Value::new(2, 0));

    let wire = Wire { value: Cell::new(Default::default()), connected_components: vec![(0, 0), (1, 0)] };

    clock.set_pin_wire(0, Some(0));
    or.set_pin_wire(0, Some(0));

    let circuit = Circuit {
        components: vec![or, clock],
        wires: vec![wire],
        clock_generators: vec![1],
        input_pins: vec![],
        output_pins: vec![],
    };

    println!("{:?} {:?}", circuit.components, circuit.wires);

    circuit.propagate_all();

    println!("{:?} {:?}", circuit.components, circuit.wires);

    circuit.propagate_all();

    println!("{:?} {:?}", circuit.components, circuit.wires);

    circuit.propagate_all();

    println!("{:?} {:?}", circuit.components, circuit.wires);

    let start = Instant::now();

    for _ in 0..1_000_000 {
        circuit.propagate_all();
    }

    println!("{:?} {:?} MHz", start.elapsed(), 1f64 / (start.elapsed().as_micros() as f64 / 1_000_000f64));
}

#[test]
pub fn test_not() {
    let clock = ClockGenerator::create();
    let not = NotGate::from_bit_width(2);

    let wire = Wire { value: Cell::new(Default::default()), connected_components: vec![(0, 0), (1, 0)] };

    clock.set_pin_wire(0, Some(0));
    not.set_pin_wire(0, Some(0));

    let circuit = Circuit {
        components: vec![clock, not],
        wires: vec![wire],
        clock_generators: vec![0],
        input_pins: vec![],
        output_pins: vec![],
    };

    println!("{:?} {:?}", circuit.components, circuit.wires);

    circuit.propagate_all();

    println!("{:?} {:?}", circuit.components, circuit.wires);

    circuit.propagate_all();

    println!("{:?} {:?}", circuit.components, circuit.wires);

    circuit.propagate_all();

    println!("{:?} {:?}", circuit.components, circuit.wires);

    let start = Instant::now();

    for _ in 0..1_000_000 {
        circuit.propagate_all();
    }

    println!("{:?} {:?} MHz", start.elapsed(), 1f64 / (start.elapsed().as_micros() as f64 / 1_000_000f64));
}