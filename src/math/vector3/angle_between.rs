use crate::math::{
    number::{Atan2, Sqrt},
    Vector3,
};
use std::ops::{Add, Mul, Sub};

impl<T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Sqrt + Atan2 + Clone> Vector3<T> {
    /// Calculates the unsigned angle (`0..2π`) between two vectors
    pub fn angle_between(self, other: Self) -> T {
        self.clone()
            .cross(other.clone())
            .length()
            .atan2(self.dot(other))
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Atan2 + Clone> Vector3<T> {
    /// Calculates the signed angle (`-π..π`) between two vectors around `normal`
    pub fn signed_angle(self, other: Self, normal: Self) -> T {
        normal
            .dot(self.clone().cross(other.clone()))
            .atan2(self.dot(other))
    }
}
