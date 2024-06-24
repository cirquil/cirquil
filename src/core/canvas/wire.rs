use std::cmp::{max, min};

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
    pub fn contains(&self, location: Location, margin: i16) -> Option<(WireIdx, usize, i16)> {
        self.segments.iter().enumerate()
            .filter_map(|(i, segment)| {
                Self::segment_contains(location, *segment, margin)
                    .and_then(|dist| {
                        Some((self.wire, i, dist))
                    })
            })
            .min_by_key(|(_, _, dist)| *dist)
    }

    pub fn projection(&self, segment: usize, location: Location) -> Location {
        let segment = self.segments[segment];
        if segment.0.x == segment.1.x {
            Location {
                x: segment.0.x,
                y: max(segment.0.y, min(location.y, segment.1.y)),
            }
        } else {
            Location {
                x: max(segment.0.x, min(location.x, segment.1.x)),
                y: segment.0.y,
            }
        }
    }

    fn segment_contains(location: Location, (start, end): (Location, Location), margin: i16) -> Option<i16> {
        let dist = if start.x == end.x {
            (start.x - location.x).abs() + max(0, max(start.y - location.y, location.y - end.y))
        } else {
            (start.y - location.y).abs() + max(0, max(start.x - location.x, location.x - end.x))
        };
        (dist <= margin).then_some(dist)
    }
}