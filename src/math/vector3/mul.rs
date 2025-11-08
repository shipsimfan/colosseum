use crate::math::Vector3;
use std::ops::{Mul, MulAssign};

impl<T: Mul<Output = T> + Clone> Mul<T> for Vector3<T> {
    type Output = Vector3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vector3::new(self.x * rhs.clone(), self.y * rhs.clone(), self.z * rhs)
    }
}

impl<T: Mul<Output = T>> Mul for Vector3<T> {
    type Output = Vector3<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Vector3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl<T: MulAssign + Clone> MulAssign<T> for Vector3<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs.clone();
        self.y *= rhs.clone();
        self.z *= rhs;
    }
}

impl<T: MulAssign> MulAssign for Vector3<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}
