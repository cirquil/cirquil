use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::canvas::location::Location;
use crate::core::simulation::pin::PinIdx;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedProbe {
    pub name: String,
    pub location: Location,
    pub subcircuit_path: Vec<Uuid>,
    pub pins: Vec<ProbePin>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProbePin {
    pub component: Uuid,
    pub pin: PinIdx,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkbenchFile {
    // checksum: (),
    pub probes: Vec<SavedProbe>,
}
