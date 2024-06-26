use serde::Deserialize;

use crate::logisim::parser::location::LogisimLocation;

#[derive(Debug, Clone, Deserialize)]
pub struct LogisimWire {
    #[serde(rename = "@from")]
    pub from: LogisimLocation,
    #[serde(rename = "@to")]
    pub to: LogisimLocation,
}
