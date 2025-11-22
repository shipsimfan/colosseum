use crate::math::{Quaternion, number::One};
use std::ops::{Add, Div, Mul, Neg};

impl<T: Add<Output = T> + Mul<Output = T> + Div<Output = T> + Neg<Output = T> + Clone + One>
    Quaternion<T>
{
    /// Calculate the inverse of this quaternion
    pub fn inverse(self) -> Self {
        let square_length = T::ONE / self.clone().length_squared();
        self.conjugate() * square_length
    }
}
