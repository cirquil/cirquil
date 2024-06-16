use serde::Deserialize;

use crate::logisim::parser::component::LogisimComponent;
use crate::logisim::parser::wire::LogisimWire;

#[derive(Debug, Deserialize)]
pub struct LogisimCircuit {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "wire")]
    pub wires: Vec<LogisimWire>,
    #[serde(rename = "comp")]
    pub components: Vec<LogisimComponent>,
}

#[derive(Debug, Deserialize)]
pub struct TopCircuit {
    #[serde(rename = "@name")]
    pub name: String,
}
