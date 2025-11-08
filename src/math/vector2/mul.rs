use crate::math::Vector2;
use std::ops::{Mul, MulAssign};

impl<T: Mul<Output = T> + Clone> Mul<T> for Vector2<T> {
    type Output = Vector2<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vector2::new(self.x * rhs.clone(), self.y * rhs)
    }
}

impl<T: Mul<Output = T>> Mul for Vector2<T> {
    type Output = Vector2<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Vector2::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl<T: MulAssign + Clone> MulAssign<T> for Vector2<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs.clone();
        self.y *= rhs;
    }
}

impl<T: MulAssign> MulAssign for Vector2<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}
