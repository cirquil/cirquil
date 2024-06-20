use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkbenchFile {
    checksum: (),
    probes: (),
}