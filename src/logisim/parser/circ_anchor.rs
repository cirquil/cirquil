use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CircAnchor {
    #[serde(rename = "@facing")]
    pub facing: String,
    #[serde(rename = "@height")]
    pub height: u32,
    #[serde(rename = "@width")]
    pub width: u32,
    #[serde(rename = "@x")]
    pub x: u32,
    #[serde(rename = "@y")]
    pub y: u32,
}
