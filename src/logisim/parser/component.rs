use serde::Deserialize;
use crate::logisim::parser::location::LogisimLocation;

#[derive(Debug, Deserialize)]
pub struct LogisimParameter {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@val", alias = "$text")]
    pub val: String,
}

fn default_lib() -> String {
    String::from("current")
}

fn default_params() -> Vec<LogisimParameter> {
    vec![]
}

#[derive(Debug, Deserialize)]
pub struct LogisimComponent {
    #[serde(rename = "@lib", default = "default_lib")]
    pub lib: String,
    #[serde(rename = "@loc")]
    pub loc: LogisimLocation,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "a", default = "default_params")]
    pub params: Vec<LogisimParameter>,
}

impl LogisimComponent {
    pub fn get_param(&self, name: &str) -> Option<&String> {
        match self.params.iter().find(|x| x.name == name)
        {
            Some(x) => Some(&x.val),
            None => None
        }
    }
}
