use crate::math::{number::Sqrt, Vector3};
use std::ops::{Add, Mul, Sub};

impl<T: Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Clone> Vector3<T> {
    /// Calculates the distance between the two vectors squared
    pub fn distance_squared(self, other: Self) -> T {
        (self - other).length_squared()
    }
}

impl<T: Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Clone + Sqrt> Vector3<T> {
    /// Calculates the distance between the two vectors
    pub fn distance(self, other: Self) -> T {
        self.distance_squared(other).sqrt()
    }
}
