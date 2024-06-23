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
pub struct OscilloscopeConfig {
    pub rows: Vec<OscilloscopeRow>,
    pub last_row_id: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OscilloscopeRow {
    pub name: String,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkbenchFile {
    pub probes: Vec<SavedProbe>,
    pub oscilloscope_config: OscilloscopeConfig,
    pub last_probe_id: usize,
}
