use crate::math::{
    number::{Atan2, Sqrt},
    Vector4,
};
use std::ops::{Add, Mul, Sub};

impl<T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Sqrt + Atan2 + Clone> Vector4<T> {
    /// Calculates the unsigned angle (`0..2π`) between two vectors
    pub fn angle_between(self, other: Self) -> T {
        let dot = self.clone().dot(other.clone());
        let wedge =
            (self.length_squared() * other.length_squared() - (dot.clone() * dot.clone())).sqrt();
        wedge.atan2(dot)
    }
}
