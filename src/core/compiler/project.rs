use std::collections::HashMap;

use crate::core::canvas::circuit::CanvasCircuit;
use crate::core::compiler::circuit::compile_circuit;
use crate::core::simulation::circuit::{Circuit, CircuitIdx};
use crate::core::simulation::component::{ComponentIdx, ComponentModel};
use crate::core::simulation::components::subcircuit::Subcircuit;
use crate::serde::project::ProjectFile;

pub struct InstantiatedCircuits {
    pub canvas_circuits: Vec<CanvasCircuit>,
    pub instantiated_circuits: Vec<(Circuit, CircuitIdx)>,
}

pub fn compile_project(project: ProjectFile) -> (CircuitIdx, InstantiatedCircuits) {
    let mut canvas_circuits: Vec<CanvasCircuit> = Vec::new();
    let mut name_to_idx: HashMap<String, CircuitIdx> = HashMap::new();
    let mut compiled_circuits: HashMap<String, Circuit> = HashMap::new();
    for (name, circ) in project.circuits.into_iter() {
        let (compiled, canvas) = compile_circuit(circ.clone());
        name_to_idx.insert(name.clone(), canvas_circuits.len());
        canvas_circuits.push(canvas);
        compiled_circuits.insert(name, compiled);
    }
    let mut instantiated_circuits: Vec<(Circuit, CircuitIdx)> = Vec::new();

    instantiate_project(project.top_circuit.as_str(),
                        &compiled_circuits, &name_to_idx,
                        &mut instantiated_circuits);

    (
        instantiated_circuits.len() - 1,
        InstantiatedCircuits {
            canvas_circuits,
            instantiated_circuits,
        }
    )
}

fn instantiate_project(name: &str,
                       compiled_circuits: &HashMap<String, Circuit>,
                       name_to_idx: &HashMap<String, CircuitIdx>,
                       instantiated_circuits: &mut Vec<(Circuit, CircuitIdx)>)
                       -> CircuitIdx {
    let mut compiled = compiled_circuits[name].clone();
    for i in compiled.components.iter_mut() {
        if let ComponentModel::Subcircuit(Subcircuit::NotInstantiated(sub_name)) = &i.component {
            let sub_idx = instantiate_project(sub_name, compiled_circuits,
                                              name_to_idx, instantiated_circuits);
            i.component = ComponentModel::Subcircuit(Subcircuit::Instantiated(sub_idx));
        }
    }

    let idx: ComponentIdx = instantiated_circuits.len();
    instantiated_circuits.push((compiled, name_to_idx[name]));
    idx
}
