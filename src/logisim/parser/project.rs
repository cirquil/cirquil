use serde::Deserialize;

use crate::logisim::parser::circuit::{LogisimCircuit, TopCircuit};

#[derive(Debug, Clone, Deserialize)]
pub struct LogisimProject {
    #[serde(rename = "circuit")]
    pub circuits: Vec<LogisimCircuit>,
    #[serde(rename = "main")]
    pub top_circuit: TopCircuit,
}