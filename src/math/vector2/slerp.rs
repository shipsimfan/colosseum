use crate::math::{
    number::{Absolute, Atan2, One, Sin},
    Vector2,
};
use std::ops::{Add, Div, Mul, Sub};

impl<
        T: One
            + Mul<Output = T>
            + Sub<Output = T>
            + Add<Output = T>
            + Div<Output = T>
            + Atan2
            + Sin
            + Absolute
            + Clone,
    > Vector2<T>
{
    /// Interpolates spherically between two vectors
    pub fn slerp(self, other: Self, t: T) -> Self {
        let angle = self.clone().angle_between(other.clone());
        let angle_sin = angle.clone().sin();

        let a = ((T::ONE - t.clone()) * angle.clone()).sin() / angle_sin.clone();
        let b = (t * angle).sin() / angle_sin;

        Vector2::new(
            self.x * a.clone() + other.x * b.clone(),
            self.y * a + other.y * b,
        )
    }
}
