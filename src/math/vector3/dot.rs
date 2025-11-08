use crate::math::Vector3;
use std::ops::{Add, Mul};

impl<T: Add<Output = T> + Mul<Output = T>> Vector3<T> {
    /// Computes the dot product of two vectors
    pub fn dot(self, other: Vector3<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}
