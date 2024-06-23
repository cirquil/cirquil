use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use serde::{Deserialize, Serialize};

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Location {
    pub x: i16,
    pub y: i16,
}

impl Location {
    pub fn new(x: i16, y: i16) -> Self {
        Location { x, y }
    }

    pub fn as_tuple(&self) -> (i16, i16) {
        (self.x, self.y)
    }
}

impl From<Location> for (i16, i16) {
    fn from(value: Location) -> Self {
        (value.x, value.y)
    }
}

impl Add for Location {
    type Output = Location;

    fn add(self, rhs: Self) -> Self::Output {
        Location::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Location {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Location {
    type Output = Location;

    fn sub(self, rhs: Self) -> Self::Output {
        Location::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Location {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<i16> for Location {
    type Output = Location;

    fn mul(self, rhs: i16) -> Self::Output {
        Location::new(self.x * rhs, self.y * rhs)
    }
}

impl MulAssign<i16> for Location {
    fn mul_assign(&mut self, rhs: i16) {
        self.x *= rhs;
        self.y *= rhs;
    }
}
