use crate::math::Color3;
use std::ops::{Mul, MulAssign};

impl<T: Mul<Output = T> + Clone> Mul<T> for Color3<T> {
    type Output = Color3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Color3::new(self.r * rhs.clone(), self.g * rhs.clone(), self.b * rhs)
    }
}

impl<T: Mul<Output = T>> Mul for Color3<T> {
    type Output = Color3<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Color3::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

impl<T: MulAssign + Clone> MulAssign<T> for Color3<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.r *= rhs.clone();
        self.g *= rhs.clone();
        self.b *= rhs;
    }
}

impl<T: MulAssign> MulAssign for Color3<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
    }
}
