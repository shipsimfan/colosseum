use crate::math::Vector3;
use std::ops::{Mul, Sub};

impl<T: Mul<Output = T> + Sub<Output = T> + Clone> Vector3<T> {
    /// Get the cross product of two [`Vector3`]s
    pub fn cross(self, other: Self) -> Self {
        Vector3::new(
            self.y.clone() * other.z.clone() - self.z.clone() * other.y.clone(),
            self.z * other.x.clone() - self.x.clone() * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}
