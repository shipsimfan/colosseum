use crate::math::{
    number::{One, Sqrt},
    Vector2,
};
use std::ops::{Add, DivAssign, Mul};

impl<T: Add<Output = T> + Mul<Output = T> + DivAssign + Clone + Sqrt + PartialEq + One> Vector2<T> {
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
    pub fn normalized(mut self) -> Vector2<T> {
        self.normalize();
        self
    }
}

impl<T: Add<Output = T> + Mul<Output = T> + Clone + PartialEq + One> Vector2<T> {
    /// Is this vector normalized?
    pub fn is_normalized(&self) -> bool {
        self.clone().length_squared() == T::ONE
    }
}
