use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::canvas::circuit::CanvasCircuit;
use crate::core::compiler::project::SimulationTreeRoot;
use crate::core::simulation::circuit::{Circuit, CircuitIdx};
use crate::core::simulation::component::ComponentIdx;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayFile {
    pub top_circuit: CircuitIdx,

    pub states: Vec<Vec<(Circuit, CircuitIdx)>>,

    pub canvas_circuits: Vec<CanvasCircuit>,
    pub simulation_tree: SimulationTreeRoot,
    pub by_uuid: Vec<HashMap<Uuid, ComponentIdx>>,
    pub parents: Vec<Option<(CircuitIdx, ComponentIdx)>>,
}
