use serde::Deserialize;

use crate::logisim::parser::location::LogisimLocation;

#[derive(Debug, Clone, Deserialize)]
pub struct LogisimParameter {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@val", alias = "$text")]
    pub val: String,
}

fn default_params() -> Vec<LogisimParameter> {
    vec![]
}

#[derive(Debug, Clone, Deserialize)]
pub struct LogisimComponent {
    #[serde(rename = "@lib")]
    pub lib: Option<u32>,
    #[serde(rename = "@loc")]
    pub loc: LogisimLocation,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "a", default = "default_params")]
    pub params: Vec<LogisimParameter>,
}

impl LogisimComponent {
    pub fn get_param(&self, name: &str) -> Option<&str> {
        match self.params.iter().find(|x| x.name == name) {
            Some(x) => Some(x.val.as_str()),
            None => None,
        }
    }
}
