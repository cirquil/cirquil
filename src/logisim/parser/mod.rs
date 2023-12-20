use std::fs::File;
use std::io::Read;
use std::path::Path;
use quick_xml::de;
use crate::logisim::parser::project::LogisimProject;

pub mod circuit;
pub mod component;
pub mod location;
pub mod wire;
pub mod project;

pub fn parse_logisim<P>(f: P) -> LogisimProject
    where
        P: AsRef<Path>,
{
    let mut xml = File::open(f).expect("File invalid");
    let mut contents = String::new();

    xml.read_to_string(&mut contents).expect("Wrong file contents.");

    let doc: LogisimProject = de::from_str(&contents).unwrap();

    doc
}