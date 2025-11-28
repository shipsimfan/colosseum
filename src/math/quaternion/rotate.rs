use crate::math::{
    Quaternion, Vector3,
    number::{One, Zero},
};
use std::ops::{Add, Div, Mul, Neg, Sub};

impl<
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Neg<Output = T>
        + Clone
        + One
        + Zero,
> Quaternion<T>
{
    /// Rotate `v` by this quaternion
    pub fn rotate(self, v: Vector3<T>) -> Vector3<T> {
        let v = Quaternion::new(v.x, v.y, v.z, T::ZERO);
        let inverse = self.clone().inverse();

        let result = (self * v) * inverse;

        Vector3::new(result.x, result.y, result.z)
    }

    /// Rotate `v` by this quaternion, assuming it is a unit quaternion
    pub fn rotate_unit(self, v: Vector3<T>) -> Vector3<T> {
        let v = Quaternion::new(v.x, v.y, v.z, T::ZERO);
        let inverse = self.clone().conjugate();

        let result = (inverse * v) * self;

        Vector3::new(result.x, result.y, result.z)
    }
}
