use self::circuit::Circuit;
use quick_xml::de;
use serde::Deserialize;
use std::{fs::File, io::Read};

pub mod circuit;

#[derive(Debug, Deserialize)]
pub struct Project {
    #[serde(rename = "circuit")]
    circuits: Vec<Circuit>,
}

fn main() {
    let doc = parse_logisim();
    println!("{:?}", doc);
}

pub fn parse_logisim() -> Project {
    let mut xml = File::open("cdm16.circ")
        .expect("There should be a file named 'cdm16.circ' in project directory.");
    let mut contents = String::new();
    xml.read_to_string(&mut contents)
        .expect("Wrong file contents.");

    let doc: Project = de::from_str(&contents).unwrap();
    doc
}