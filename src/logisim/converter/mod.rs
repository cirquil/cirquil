use self::circuit::Circuit;
use quick_xml::de;
use serde::Deserialize;
use std::{fs::File, io::Read, path::Path};

pub mod circuit;

#[derive(Debug, Deserialize)]
pub struct Project {
    #[serde(rename = "circuit")]
    pub circuits: Vec<Circuit>,
}

pub fn parse_logisim<P>(f: P) -> Project
where
    P: AsRef<Path>,
{
    let mut xml = File::open(f).expect("File invalid");
    let mut contents = String::new();
    xml.read_to_string(&mut contents)
        .expect("Wrong file contents.");

    let doc: Project = de::from_str(&contents).unwrap();
    doc
}
