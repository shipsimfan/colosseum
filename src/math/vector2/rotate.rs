use crate::math::{
    number::{Cos, Sin},
    Vector2,
};
use std::ops::{Add, Mul, Neg, Sub};

impl<T: Neg<Output = T> + Clone> Vector2<T> {
    /// Rotate this vector 90 degrees clockwise
    pub fn rotate_cw(self) -> Self {
        Vector2::new(self.y, -self.x)
    }

    /// Rotate this vector 90 degrees counter-clockwise
    pub fn rotate_ccw(self) -> Vector2<T> {
        Vector2::new(-self.y, self.x)
    }
}

impl<T: Cos + Sin + Mul<Output = T> + Sub<Output = T> + Add<Output = T> + Clone> Vector2<T> {
    /// Rotate this vector by `angle` radians
    pub fn rotate(self, angle: T) -> Self {
        let sin_angle = angle.clone().sin();
        let cos_angle = angle.cos();
        Vector2::new(
            cos_angle.clone() * self.x.clone() - sin_angle.clone() * self.y.clone(),
            sin_angle * self.x + cos_angle * self.y,
        )
    }
}
