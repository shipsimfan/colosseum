use crate::math::Vector2;
use std::ops::{Mul, Sub};

impl<T: Mul<Output = T> + Sub<Output = T>> Vector2<T> {
    /// Get the 2d-cross product
    ///
    /// The result is equal to the area of the parallelogram formed by the two vectors
    pub fn cross_2d(self, other: Self) -> T {
        self.x * other.y - self.y * other.x
    }
}
