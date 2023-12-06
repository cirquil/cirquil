use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Wire {
    #[serde(rename = "@from")]
    from: String,
    #[serde(rename = "@to")]
    to: String,
}
