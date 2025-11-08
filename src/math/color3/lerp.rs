use crate::math::{
    number::{One, Zero},
    Color3,
};
use std::ops::{Add, Mul, Sub};

impl<T: Zero + One + PartialOrd + Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Clone>
    Color3<T>
{
    /// Interpolates linearly between two vectors, clamping `t` between 0 and 1
    pub fn lerp(self, other: Self, t: T) -> Self {
        let t = if t < T::ZERO {
            T::ZERO
        } else if t > T::ONE {
            T::ONE
        } else {
            t
        };
        self.lerp_unclamped(other, t)
    }

    /// Interpolates linearly between two vectors, without clamping `t` between 0 and 1
    pub fn lerp_unclamped(self, other: Self, t: T) -> Self {
        Color3::new(
            self.r.clone() + t.clone() * (other.r - self.r),
            self.g.clone() + t.clone() * (other.g - self.g),
            self.b.clone() + t * (other.b - self.b),
        )
    }
}
