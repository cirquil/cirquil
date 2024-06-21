use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::io::Error as StdIoError;
use std::path::Path;

use crate::core::compiler::project::compile_project;
use crate::logisim::converter::convert_logisim_project;
use crate::logisim::parser::parse_logisim;
use crate::player::CirquilPlayerApp;
use crate::serde::project::ProjectFile;

#[derive(Debug)]
pub enum LoadErrorKind {
    UnknownFileType,
    IoError(StdIoError),
    ConvertError,
    UnknownError,
}

#[derive(Debug)]
pub struct ProjectLoadError {
    pub kind: LoadErrorKind,
}

impl Display for ProjectLoadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("Can't load project {:?}", self.kind).as_str())
    }
}

impl Error for ProjectLoadError {}

impl From<LoadErrorKind> for ProjectLoadError {
    fn from(value: LoadErrorKind) -> Self {
        ProjectLoadError { kind: value }
    }
}

impl From<StdIoError> for ProjectLoadError {
    fn from(value: StdIoError) -> Self {
        ProjectLoadError {
            kind: LoadErrorKind::IoError(value)
        }
    }
}

impl CirquilPlayerApp {
    pub fn load_project<P>(&mut self, path: P) -> Result<(), ProjectLoadError>
        where P: AsRef<Path> {
        let project_file = match path.as_ref().extension() {
            None => { return Err(ProjectLoadError::from(LoadErrorKind::UnknownFileType)); }
            Some(e) => {
                match e.to_str() {
                    Some("cirq") => {
                        ProjectFile::load(path)?
                    }
                    Some("circ") => {
                        let logisim_project = parse_logisim(path)
                            .map_err(|_| ProjectLoadError::from(LoadErrorKind::UnknownError))?;

                        convert_logisim_project(logisim_project)
                    }
                    Some(_) => { return Err(ProjectLoadError::from(LoadErrorKind::UnknownFileType)); }
                    None => {
                        return Err(ProjectLoadError::from(LoadErrorKind::UnknownError));
                    }
                }
            }
        };

        let (top_circuit, compiled_circuits) = compile_project(project_file);

        compiled_circuits.instantiated_circuits.iter().for_each(
            |(circuit, _)| circuit.propagate_all()
        );

        self.circuits = compiled_circuits;
        self.top_circuit = top_circuit;
        self.current_circuit = top_circuit;

        Ok(())
    }
}