use crate::core::compiler::project::InstantiatedCircuits;
use crate::core::simulation::probe::CanvasProbe;
use crate::serde::workbench::{SavedProbe, WorkbenchFile};

impl CanvasProbe {
    fn from_saved(saved: SavedProbe,
                  circuits: &InstantiatedCircuits)
                  -> Result<CanvasProbe, String> {
        return Err(String::from("aa"));
    }
    fn to_saved(&self, circuits: &InstantiatedCircuits)
                -> SavedProbe {
        return SavedProbe {
            name: self.probe.name.clone(),
            location: self.location,
            subcircuit_path: vec![],
            pins: vec![],
        };
    }
}

pub fn from_workbench_file(workbench_file: WorkbenchFile,
                           circuits: &InstantiatedCircuits)
                           -> Result<Vec<CanvasProbe>, String> {
    let canvas_probes: Result<Vec<CanvasProbe>, String> = workbench_file.probes
        .into_iter()
        .map(|x| CanvasProbe::from_saved(x, circuits))
        .collect();

    canvas_probes
}

pub fn to_workbench_file(canvas_probes: &Vec<CanvasProbe>,
                         circuits: &InstantiatedCircuits)
                         -> WorkbenchFile {
    let saved_probes: Vec<SavedProbe> = canvas_probes
        .iter()
        .map(|x| CanvasProbe::to_saved(x, circuits))
        .collect();

    WorkbenchFile {
        probes: saved_probes,
    }
}
