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