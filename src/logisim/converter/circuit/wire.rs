use serde::Deserialize;
use crate::logisim::converter::circuit::point::Point;

#[derive(Debug, Deserialize)]
pub struct Wire {
    #[serde(rename = "@from")]
    from: Point,
    #[serde(rename = "@to")]
    to: Point,
}
