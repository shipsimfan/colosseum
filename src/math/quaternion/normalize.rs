use crate::math::{
    number::{One, Sqrt},
    Quaternion,
};
use std::ops::{Add, DivAssign, Mul};

impl<T: Add<Output = T> + Mul<Output = T> + DivAssign + Clone + Sqrt + PartialEq + One> Quaternion<T> {
    /// Normalizes the values in the vector to make its length 1
    pub fn normalize(&mut self) {
        let length_squared = self.clone().length_squared();
        if length_squared == T::ONE {
            return;
        }

        let length = length_squared.sqrt();
        *self /= length;
    }

    /// Gets a normalized version of this vector, such that the length is 1
    pub fn normalized(mut self) -> Quaternion<T> {
        self.normalize();
        self
    }
}

impl<T: Add<Output = T> + Mul<Output = T> + Clone + PartialEq + One> Quaternion<T> {
    /// Is this vector normalized?
    pub fn is_normalized(&self) -> bool {
        self.clone().length_squared() == T::ONE
    }
}
