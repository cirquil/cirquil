use std::collections::HashMap;
use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::core::canvas::circuit::CanvasCircuit;
use crate::core::compiler::circuit::compile_circuit;
use crate::core::simulation::circuit::{Circuit, CircuitIdx};
use crate::core::simulation::component::{ComponentIdx, ComponentModel};
use crate::core::simulation::components::subcircuit::Subcircuit;
use crate::serde::project::ProjectFile;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SimulationTreeNode {
    Leaf(CircuitIdx),
    Node(CircuitIdx, Vec<SimulationTreeNode>),
}

pub type SimulationTreeRoot = SimulationTreeNode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstantiatedCircuits {
    pub canvas_circuits: Vec<CanvasCircuit>,
    pub instantiated_circuits: Vec<(Rc<Circuit>, CircuitIdx)>,
    pub simulation_tree: SimulationTreeRoot,
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
    let mut instantiated_circuits: Vec<(Rc<Circuit>, CircuitIdx)> = Vec::new();

    instantiate_project(project.top_circuit.as_str(),
                        &compiled_circuits, &name_to_idx,
                        &mut instantiated_circuits);

    // TODO: Implement this
    let simulation_tree = SimulationTreeNode::Leaf(0);
    
    (
        instantiated_circuits.len() - 1,
        InstantiatedCircuits {
            canvas_circuits,
            instantiated_circuits,
            simulation_tree,
        }
    )
}

fn instantiate_project(name: &str,
                       compiled_circuits: &HashMap<String, Circuit>,
                       name_to_idx: &HashMap<String, CircuitIdx>,
                       instantiated_circuits: &mut Vec<(Rc<Circuit>, CircuitIdx)>)
                       -> CircuitIdx {
    let mut compiled = compiled_circuits[name].clone();
    for i in compiled.components.iter_mut() {
        if let ComponentModel::Subcircuit(Subcircuit::NotInstantiated(sub_name)) = &i.model {
            let sub_idx = instantiate_project(sub_name, compiled_circuits,
                                              name_to_idx, instantiated_circuits);

            let (circuit, _) = instantiated_circuits.get(sub_idx).unwrap();

            i.model =
                ComponentModel::Subcircuit(
                    Subcircuit::Instantiated(circuit.clone())
                );
        }
    }

    let idx: ComponentIdx = instantiated_circuits.len();
    instantiated_circuits.push((Rc::new(compiled), name_to_idx[name]));
    idx
}
