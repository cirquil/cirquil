use serde::Deserialize;
use crate::logisim::converter::circuit::point::Point;

#[derive(Debug, Deserialize)]
pub struct Param {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@val", alias = "$text")]
    pub val: String,
}

fn default_lib() -> String {
    String::from("current")
}

fn default_params() -> Vec<Param> {
    vec![]
}

#[derive(Debug, Deserialize)]
pub struct Component {
    #[serde(rename = "@lib", default = "default_lib")]
    pub lib: String,
    #[serde(rename = "@loc")]
    pub loc: Point,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "a", default = "default_params")]
    pub params: Vec<Param>,
}
