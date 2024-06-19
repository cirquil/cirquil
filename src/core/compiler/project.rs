use std::collections::HashMap;

use crate::core::canvas::circuit::CanvasCircuit;
use crate::core::compiler::circuit::compile_circuit;
use crate::core::simulation::circuit::{Circuit, CircuitIdx};
use crate::serde::project::ProjectFile;

pub struct InstantiatedCircuits {
    pub canvas_circuits: Vec<CanvasCircuit>,
    pub instantiated_circuits: Vec<(Circuit, CircuitIdx)>,
}

pub fn compile_project(project: ProjectFile) -> InstantiatedCircuits {
    let compiled_circuits: HashMap<String, (Circuit, CanvasCircuit)> = project.circuits.clone().into_iter()
        .map(|(name, circ)| (name, compile_circuit(circ)))
        .collect();

    let (root_circuit, _) = compiled_circuits.get(project.top_circuit.as_str()).unwrap();

    let circuit_tree = vec![root_circuit.clone()];

    /*

    Run DFS from root circuit, for each not instantiated circuit clone it to vector 

     */

    InstantiatedCircuits {
        canvas_circuits: vec![],
        instantiated_circuits: vec![],
    }
}
