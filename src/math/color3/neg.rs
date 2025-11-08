use crate::math::Color3;
use std::ops::Neg;

impl<T: Neg<Output = T>> Neg for Color3<T> {
    type Output = Color3<T>;

    fn neg(self) -> Self::Output {
        Color3::new(-self.r, -self.g, -self.b)
    }
}
