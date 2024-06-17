use serde::Deserialize;

use crate::logisim::parser::location::LogisimLocation;

#[derive(Debug, Deserialize)]
pub struct CircPort {
    #[serde(rename = "@height")]
    pub height: u32,
    #[serde(rename = "@pin")]
    pub pin: LogisimLocation,
    #[serde(rename = "@width")]
    pub width: u32,
    #[serde(rename = "@x")]
    pub x: u32,
    #[serde(rename = "@y")]
    pub y: u32,
}
