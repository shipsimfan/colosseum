use crate::math::Vector4;
use std::ops::Neg;

impl<T: Neg<Output = T>> Neg for Vector4<T> {
    type Output = Vector4<T>;

    fn neg(self) -> Self::Output {
        Vector4::new(-self.x, -self.y, -self.z, -self.w)
    }
}
