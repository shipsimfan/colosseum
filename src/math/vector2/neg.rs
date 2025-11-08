use crate::math::Vector2;
use std::ops::Neg;

impl<T: Neg<Output = T>> Neg for Vector2<T> {
    type Output = Vector2<T>;

    fn neg(self) -> Self::Output {
        Vector2::new(-self.x, -self.y)
    }
}
