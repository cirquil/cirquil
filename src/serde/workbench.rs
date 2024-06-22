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
