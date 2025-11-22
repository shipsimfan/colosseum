use crate::math::Quaternion;
use std::ops::Neg;

impl<T: Neg<Output = T>> Quaternion<T> {
    /// Get the complex conjugate of this [`Quaternion`]
    pub fn conjugate(self) -> Self {
        Quaternion::new(-self.x, -self.y, -self.z, self.w)
    }
}
