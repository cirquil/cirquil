use std::path::Path;

use crate::core::simulation::probe::CanvasProbe;
use crate::core::simulation::workbench::{from_workbench_file, to_workbench_file};
use crate::player::CirquilPlayerApp;
use crate::serde::fs::{deserialize_from_file, serialize_to_file};
use crate::serde::workbench::WorkbenchFile;

impl CirquilPlayerApp {
    pub fn load_workbench<P>(&mut self, path: P) -> Option<Vec<String>>
        where P: AsRef<Path>
    {
        let workbench_file: WorkbenchFile = deserialize_from_file(path).unwrap();

        let probes = from_workbench_file(workbench_file, &self.circuits);

        let verified_probes: Vec<CanvasProbe> = probes.iter()
            .filter(|probe| probe.is_ok())
            .map(|probe| probe.clone().unwrap())
            .collect();

        let failed_probe_errors: Vec<String> = probes.into_iter()
            .filter(|probe| probe.is_err())
            .map(|probe| probe.unwrap_err())
            .collect();

        self.probes = verified_probes;

        if failed_probe_errors.is_empty() {
            None
        } else {
            Some(failed_probe_errors)
        }
    }

    pub fn save_workbench<P>(&mut self, path: P)
        where P: AsRef<Path>
    {
        let workbench_file = to_workbench_file(&self.probes, &self.circuits);
        serialize_to_file(&workbench_file, path).unwrap();
    }
}