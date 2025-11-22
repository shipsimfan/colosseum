use crate::math::{Quaternion, Vector3, number::One};
use std::ops::{Add, Mul, Sub};

impl<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + One + Clone> Quaternion<T> {
    /// Get the right basis vector
    pub fn right(self) -> Vector3<T> {
        let two = T::ONE + T::ONE;

        Vector3::new(
            T::ONE
                - two.clone() * self.y.clone() * self.y.clone()
                - two.clone() * self.z.clone() * self.z.clone(),
            two.clone() * self.x.clone() * self.y.clone()
                + two.clone() * self.w.clone() * self.z.clone(),
            two.clone() * self.x * self.z - two * self.w * self.y,
        )
    }

    /// Get the up basis vector
    pub fn up(self) -> Vector3<T> {
        let two = T::ONE + T::ONE;

        Vector3::new(
            two.clone() * self.x.clone() * self.y.clone()
                - two.clone() * self.w.clone() * self.z.clone(),
            T::ONE
                - two.clone() * self.x.clone() * self.x.clone()
                - two.clone() * self.z.clone() * self.z.clone(),
            two.clone() * self.y * self.z + two * self.w * self.x,
        )
    }

    /// Get the forward basis vector
    pub fn forward(self) -> Vector3<T> {
        let two = T::ONE + T::ONE;

        Vector3::new(
            two.clone() * self.x.clone() * self.z.clone()
                + two.clone() * self.w.clone() * self.y.clone(),
            two.clone() * self.y.clone() * self.z - two.clone() * self.w * self.x.clone(),
            T::ONE - two.clone() * self.x.clone() * self.x - two * self.y.clone() * self.y,
        )
    }
}
