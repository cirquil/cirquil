use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::core::canvas::location::Location;
use crate::core::simulation::probe::Probe;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedProbe {
    pub location: Location,
    pub probe: Probe,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkbenchFile {
    // checksum: (),
    pub probes: Vec<SavedProbe>,
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
