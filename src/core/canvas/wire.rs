use serde::{Deserialize, Serialize};

use crate::core::canvas::location::Location;
use crate::core::simulation::wire::WireIdx;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasWire {
    pub wire: WireIdx,
    pub segments: Vec<(Location, Location)>,
    pub nodes: Vec<Location>,
}

impl CanvasWire {
    pub fn contains(&self, location: Location, margin: i16) -> bool {
        self.segments.iter()
            .any(|segment| Self::segment_contains(location, *segment, margin))
    }

    fn segment_contains(location: Location, (start, end): (Location, Location), margin: i16) -> bool {
        start.x - margin <= location.x && location.x <= end.x + margin
            && start.y - margin <= location.y && location.y <= end.y + margin
    }
}