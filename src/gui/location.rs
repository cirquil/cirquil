use egui::{Pos2, Vec2};

use crate::core::canvas::location::Location;

impl From<Location> for Vec2 {
    fn from(value: Location) -> Self {
        Self { x: value.x as f32, y: value.y as f32 }
    }
}

impl From<Location> for Pos2 {
    fn from(value: Location) -> Self {
        Self { x: value.x as f32, y: value.y as f32 }
    }
}

impl From<Pos2> for Location {
    fn from(value: Pos2) -> Self {
        Location::new(value.x as i16, value.y as i16)
    }
}

impl From<Vec2> for Location {
    fn from(value: Vec2) -> Self {
        Location::new(value.x as i16, value.y as i16)
    }
}
