use serde::Deserialize;
use crate::logisim::converter::circuit::point::Point;

#[derive(Debug, Deserialize)]
pub struct Wire {
    #[serde(rename = "@from")]
    pub from: Point,
    #[serde(rename = "@to")]
    pub to: Point,
}
