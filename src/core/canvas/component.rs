use serde::{Deserialize, Serialize};

use crate::core::canvas::location::Location;
use crate::core::simulation::component::ComponentIdx;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasComponent {
    pub component: ComponentIdx,
    pub loc: Location,
}