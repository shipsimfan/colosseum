use crate::math::Vector4;
use std::ops::{Mul, MulAssign};

impl<T: Mul<Output = T> + Clone> Mul<T> for Vector4<T> {
    type Output = Vector4<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vector4::new(
            self.x * rhs.clone(),
            self.y * rhs.clone(),
            self.z * rhs.clone(),
            self.w * rhs,
        )
    }
}

impl<T: Mul<Output = T>> Mul for Vector4<T> {
    type Output = Vector4<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Vector4::new(
            self.x * rhs.x,
            self.y * rhs.y,
            self.z * rhs.z,
            self.w * rhs.w,
        )
    }
}

impl<T: MulAssign + Clone> MulAssign<T> for Vector4<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs.clone();
        self.y *= rhs.clone();
        self.z *= rhs.clone();
        self.w *= rhs;
    }
}

impl<T: MulAssign> MulAssign for Vector4<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
        self.w *= rhs.w;
    }
}
