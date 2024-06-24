use std::error::Error;
use std::path::Path;

use colored::Colorize;

use crate::core::compiler::project::{compile_project, InstantiatedCircuits};
use crate::core::simulation::circuit::CircuitIdx;
use crate::core::simulation::probe::CanvasProbe;
use crate::core::simulation::workbench::from_workbench_file;
use crate::serde::fs::deserialize_from_file;
use crate::serde::project::ProjectFile;
use crate::serde::workbench::WorkbenchFile;

pub fn load_circuit<P>(path: P) -> Result<(CircuitIdx, InstantiatedCircuits), Box<dyn Error>>
    where
        P: AsRef<Path>
{
    let project_file = ProjectFile::load(path)?;

    let (top_circuit, compiled_circuits) = compile_project(project_file);

    compiled_circuits.instantiated_circuits.iter().for_each(
        |(circuit, _)| circuit.propagate_all()
    );

    Ok((top_circuit, compiled_circuits))
}

pub fn load_probes<P>(path: P, compiled_circuits: &InstantiatedCircuits) -> Result<Vec<CanvasProbe>, Box<dyn Error>>
    where
        P: AsRef<Path>
{
    let workbench_file: WorkbenchFile = deserialize_from_file(path)?;

    let (probes, _, _) = from_workbench_file(workbench_file, compiled_circuits);

    let verified_probes: Vec<CanvasProbe> = probes.iter()
        .filter_map(|probe| probe.clone().ok())
        .collect();

    probes.into_iter()
        .filter(|probe| probe.is_err())
        .map(|probe| probe.unwrap_err())
        .for_each(|error| println!("{}: {error}", "WARNING".yellow()));


    Ok(verified_probes)
}