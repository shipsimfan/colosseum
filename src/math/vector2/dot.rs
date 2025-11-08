use crate::math::Vector2;
use std::ops::{Add, Mul};

impl<T: Add<Output = T> + Mul<Output = T>> Vector2<T> {
    /// Computes the dot product of two vectors
    pub fn dot(self, other: Vector2<T>) -> T {
        self.x * other.x + self.y * other.y
    }
}
