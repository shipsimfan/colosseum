use crate::math::Quaternion;
use std::ops::{Add, Mul};

impl<T: Add<Output = T> + Mul<Output = T>> Quaternion<T> {
    /// Computes the dot product of two vectors
    pub fn dot(self, other: Quaternion<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }
}
