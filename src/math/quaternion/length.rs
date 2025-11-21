use crate::math::{Quaternion, number::Sqrt};
use std::ops::{Add, Mul};

impl<T: Add<Output = T> + Mul<Output = T> + Clone> Quaternion<T> {
    /// Calculates the length of the vector squared
    pub fn length_squared(self) -> T {
        self.clone().dot(self)
    }
}

impl<T: Add<Output = T> + Mul<Output = T> + Clone + Sqrt> Quaternion<T> {
    /// Calculates the length of the vector
    pub fn length(self) -> T {
        self.length_squared().sqrt()
    }
}
