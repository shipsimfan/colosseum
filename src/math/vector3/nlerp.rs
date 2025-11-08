use crate::math::{
    number::{One, Sqrt, Zero},
    Vector3,
};
use std::ops::{Add, DivAssign, Mul, Sub};

impl<
        T: Zero
            + One
            + PartialOrd
            + Add<Output = T>
            + Mul<Output = T>
            + Sub<Output = T>
            + DivAssign
            + Sqrt
            + Clone,
    > Vector3<T>
{
    /// Interpolates linearly between two vectors, normalizing the result
    ///
    /// `t` will be clamped between 0 and 1
    pub fn nlerp(self, other: Self, t: T) -> Self {
        self.lerp(other, t).normalized()
    }

    /// Interpolates linearly between two vectors, normalizing the result
    ///
    /// `t` will not be clamped between 0 and 1
    pub fn nlerp_unclamped(self, other: Self, t: T) -> Self {
        self.lerp_unclamped(other, t).normalized()
    }
}
