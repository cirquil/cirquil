use crate::circuit::{component::Component, wire::Wire};
use serde::Deserialize;

pub mod component;
pub mod wire;

#[derive(Debug, Deserialize)]
pub struct Circuit {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "wire")]
    wires: Vec<Wire>,
    #[serde(rename = "comp")]
    components: Vec<Component>,
}
