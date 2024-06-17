use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::logisim::parser::project::LogisimProject;

pub mod circuit;
pub mod component;
pub mod location;
pub mod wire;
pub mod project;
pub mod appear;
pub mod circ_port;
pub mod rect;

pub fn parse_logisim<P>(path: P) -> Result<LogisimProject, Box<dyn Error>>
    where
        P: AsRef<Path>,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let deserialized = quick_xml::de::from_reader(reader)?;

    Ok(deserialized)
}