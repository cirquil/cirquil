use serde::Deserialize;
use crate::logisim::parser::circuit::LogisimCircuit;

#[derive(Debug, Deserialize)]
pub struct LogisimProject {
    #[serde(rename = "circuit")]
    pub circuits: Vec<LogisimCircuit>,
}