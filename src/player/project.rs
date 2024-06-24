use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::io::Error as StdIoError;
use std::path::{Path, PathBuf};

use crate::core::compiler::project::compile_project;
use crate::logisim::converter::convert_logisim_project;
use crate::logisim::parser::parse_logisim;
use crate::player::circuit::CircuitManager;
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

fn load_from_file<P>(path: P) -> Result<ProjectFile, ProjectLoadError>
where
    P: AsRef<Path>,
{
    match path.as_ref().extension() {
        None => { Err(ProjectLoadError::from(LoadErrorKind::UnknownFileType)) }
        Some(e) => {
            match e.to_str() {
                Some("cirq") => {
                    Ok(ProjectFile::load(path)?)
                }
                Some("circ") => {
                    let logisim_project = parse_logisim(path)
                        .map_err(|_| ProjectLoadError::from(LoadErrorKind::UnknownError))?;

                    Ok(convert_logisim_project(logisim_project))
                }
                Some(_) => { Err(ProjectLoadError::from(LoadErrorKind::UnknownFileType)) }
                None => { Err(ProjectLoadError::from(LoadErrorKind::UnknownError)) }
            }
        }
    }
}

impl CirquilPlayerApp {
    pub fn load_project<P>(&mut self, path: P) -> Result<(), ProjectLoadError>
    where
        P: AsRef<Path>,
    {
        let project_file = load_from_file(path)?;

        let (top_circuit, compiled_circuits) = compile_project(project_file);

        compiled_circuits.instantiated_circuits.iter().for_each(
            |(circuit, _)| circuit.propagate_all()
        );

        self.circuit_manager = CircuitManager::create_simulation(compiled_circuits);
        self.top_circuit = top_circuit;
        self.current_circuit = top_circuit;
        self.probes = vec![];
        self.probe_max_id = 0;

        Ok(())
    }

    pub fn convert<P>(&mut self, path: P, cirq_path: P) -> Result<(), ProjectLoadError>
    where
        P: AsRef<Path>,
    {
        let project_file = load_from_file(path)?;
        project_file.save(cirq_path)?;

        Ok(())
    }
}


pub fn show_load_project_file_dialog() -> Option<PathBuf> {
    rfd::FileDialog::new()
        .add_filter("All supported formats", vec!["cirq", "circ"].as_slice())
        .add_filter("Cirquil Project", vec!["cirq"].as_slice())
        .add_filter("Logisim Project", vec!["circ"].as_slice())
        .pick_file()
}

pub fn show_load_logisim_file_dialog() -> Option<PathBuf> {
    rfd::FileDialog::new()
        .add_filter("Logisim Project", vec!["circ"].as_slice())
        .pick_file()
}

pub fn show_save_project_file_dialog() -> Option<PathBuf> {
    rfd::FileDialog::new()
        .add_filter("Cirquil Project", vec!["cirq"].as_slice())
        .save_file()
}
