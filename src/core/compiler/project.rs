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
    let mut compiled_circuits: Vec<Circuit> = Vec::new();
    let mut name_to_idx: HashMap<String, CircuitIdx> = HashMap::new();
    for (name, circ) in project.circuits.into_iter() {
        let (compiled, canvas) = compile_circuit(name.clone(), circ.clone());
        name_to_idx.insert(name, canvas_circuits.len());
        canvas_circuits.push(canvas);
        compiled_circuits.push(compiled);
    }
    let mut instantiated_circuits: Vec<(Rc<Circuit>, CircuitIdx)> = Vec::new();

    let simulation_tree = instantiate_tree(project.top_circuit.as_str(), &name_to_idx,
                                           &compiled_circuits,
                                           &mut instantiated_circuits);
    (
        instantiated_circuits.len() - 1,
        InstantiatedCircuits {
            canvas_circuits,
            instantiated_circuits,
            simulation_tree,
        }
    )
}

fn instantiate_tree(name: &str,
                    name_to_idx: &HashMap<String, CircuitIdx>,
                    compiled_circuits: &Vec<Circuit>,
                    instantiated_circuits: &mut Vec<(Rc<Circuit>, CircuitIdx)>)
                    -> SimulationTreeNode {
    let circuit_idx = name_to_idx[name];
    let mut compiled = compiled_circuits[circuit_idx].clone();
    let mut children_trees: Vec<SimulationTreeNode> = Vec::new();

    for i in compiled.components.iter_mut() {
        if let ComponentModel::Subcircuit(Subcircuit::NotInstantiated(sub_name)) = &i.model {
            let sub_tree = instantiate_tree(sub_name,
                                            name_to_idx, compiled_circuits, instantiated_circuits);
            let sub_idx = match sub_tree {
                SimulationTreeNode::Leaf(idx) => { idx }
                SimulationTreeNode::Node(idx, _) => { idx }
            };
            children_trees.push(sub_tree);
            let (circuit, _) = instantiated_circuits.get(sub_idx).unwrap();
            i.model =
                ComponentModel::Subcircuit(
                    Subcircuit::Instantiated(circuit.clone())
                );
        }
    }

    let idx: ComponentIdx = instantiated_circuits.len();
    instantiated_circuits.push((Rc::new(compiled), circuit_idx));
    if children_trees.is_empty() {
        SimulationTreeNode::Leaf(idx)
    } else {
        SimulationTreeNode::Node(idx, children_trees)
    }
}
