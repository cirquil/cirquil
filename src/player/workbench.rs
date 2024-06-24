use std::path::{Path, PathBuf};

use crate::core::simulation::probe::CanvasProbe;
use crate::core::simulation::workbench::{from_workbench_file, to_workbench_file};
use crate::player::CirquilPlayerApp;
use crate::serde::fs::{deserialize_from_file, serialize_to_file};
use crate::serde::workbench::WorkbenchFile;

impl CirquilPlayerApp {
    pub fn load_workbench<P>(&mut self, path: P) -> Option<Vec<String>>
        where
            P: AsRef<Path>,
    {
        let workbench_file: WorkbenchFile = deserialize_from_file(path).unwrap();

        let (probes, osc, last_probe_id) = from_workbench_file(workbench_file, self.circuit_manager.get_circuits());
        self.osc = osc;
        self.probe_max_id = last_probe_id;

        let verified_probes: Vec<CanvasProbe> = probes.iter()
            .filter_map(|probe| probe.clone().ok())
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
        where
            P: AsRef<Path>,
    {
        let workbench_file = to_workbench_file(&self.probes, &self.osc,
                                               self.circuit_manager.get_circuits(), self.probe_max_id);
        serialize_to_file(&workbench_file, path).unwrap();
    }
}

pub fn show_load_workbench_file_dialogue() -> Option<PathBuf> {
    rfd::FileDialog::new()
        .add_filter("Cirquil Workbench", vec!["cwb"].as_slice())
        .pick_file()
}

pub fn show_save_workbench_file_dialogue() -> Option<PathBuf> {
    rfd::FileDialog::new()
        .add_filter("Cirquil Workbench", vec!["cwb"].as_slice())
        .save_file()
}

