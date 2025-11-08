use crate::math::Vector4;
use std::ops::{Add, Mul};

impl<T: Add<Output = T> + Mul<Output = T>> Vector4<T> {
    /// Computes the dot product of two vectors
    pub fn dot(self, other: Vector4<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }
}
