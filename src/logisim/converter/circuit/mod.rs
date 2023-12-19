use self::{component::Component, wire::Wire};
use serde::Deserialize;

pub mod component;
pub mod wire;
pub mod point;

#[derive(Debug, Deserialize)]
pub struct Circuit {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "wire")]
    wires: Vec<Wire>,
    #[serde(rename = "comp")]
    components: Vec<Component>,
}
