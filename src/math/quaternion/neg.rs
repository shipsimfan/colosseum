use crate::math::Quaternion;
use std::ops::Neg;

impl<T: Neg<Output = T>> Neg for Quaternion<T> {
    type Output = Quaternion<T>;

    fn neg(self) -> Self::Output {
        Quaternion::new(-self.x, -self.y, -self.z, -self.w)
    }
}
