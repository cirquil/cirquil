use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Rect {
    #[serde(rename = "@fill")]
    pub fill: String,
    #[serde(rename = "@height")]
    pub height: u32,
    #[serde(rename = "@stroke")]
    pub stroke: String,
    #[serde(rename = "@stroke-width")]
    pub stroke_width: u8,
    #[serde(rename = "@width")]
    pub width: u32,
    #[serde(rename = "@x")]
    pub x: u32,
    #[serde(rename = "@y")]
    pub y: u32,
}
