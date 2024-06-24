use serde::{Deserialize, Serialize};

use crate::core::compiler::project::InstantiatedCircuits;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayFile {
    pub states: Vec<InstantiatedCircuits>,
}
