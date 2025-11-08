use crate::math::Vector3;
use std::ops::Neg;

impl<T: Neg<Output = T>> Neg for Vector3<T> {
    type Output = Vector3<T>;

    fn neg(self) -> Self::Output {
        Vector3::new(-self.x, -self.y, -self.z)
    }
}
