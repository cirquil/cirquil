use std::ops::{Add, AddAssign};

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub struct Location(pub i16, pub i16);

impl Add for Location {
    type Output = Location;

    fn add(self, rhs: Self) -> Self::Output {
        Location(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Location {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}