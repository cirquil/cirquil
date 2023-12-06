use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Param {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@val", alias = "$text")]
    val: String,
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
    lib: String,
    #[serde(rename = "@loc")]
    loc: String,
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "a", default = "default_params")]
    params: Vec<Param>,
}
