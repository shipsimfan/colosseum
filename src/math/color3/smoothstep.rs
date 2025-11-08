use crate::math::{
    number::{One, Zero},
    Color3,
};
use std::ops::{Add, Mul, Sub};

impl<T: Zero + One + PartialOrd + Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Clone>
    Color3<T>
{
    /// Interpolates using the smoothstep algorithm between two vectors
    pub fn smoothstep(self, other: Self, t: T) -> Self {
        let t = if t < T::ZERO {
            T::ZERO
        } else if t > T::ONE {
            T::ONE
        } else {
            t.clone() * t.clone() * ((T::ONE + T::ONE + T::ONE) - (T::ONE + T::ONE) * t)
        };
        self.lerp_unclamped(other, t)
    }
}
