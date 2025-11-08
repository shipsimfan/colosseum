use crate::math::{number::Sqrt, Vector3};
use std::ops::{Add, Mul};

impl<T: Add<Output = T> + Mul<Output = T> + Clone> Vector3<T> {
    /// Calculates the length of the vector squared
    pub fn length_squared(self) -> T {
        self.clone().dot(self)
    }
}

impl<T: Add<Output = T> + Mul<Output = T> + Clone + Sqrt> Vector3<T> {
    /// Calculates the length of the vector
    pub fn length(self) -> T {
        self.length_squared().sqrt()
    }
}
