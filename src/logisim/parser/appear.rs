use serde::Deserialize;

use crate::logisim::parser::circ_port::CircPort;
use crate::logisim::parser::rect::Rect;

#[derive(Debug, Deserialize)]
pub struct Appear {
    #[serde(rename = "rect")]
    pub rect: Rect,
    #[serde(rename = "circ-port")]
    pub circ_ports: Vec<CircPort>
}
