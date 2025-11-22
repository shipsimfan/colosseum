use crate::math::{
    Quaternion, Vector3,
    number::{One, Zero},
};
use std::ops::{Add, Div, Mul, MulAssign, Neg, Sub};

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

impl<
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Neg<Output = T>
        + Clone
        + One
        + Zero,
> Mul<Quaternion<T>> for Vector3<T>
{
    type Output = Vector3<T>;

    fn mul(self, rhs: Quaternion<T>) -> Self::Output {
        rhs.rotate(self)
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

impl<
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Neg<Output = T>
        + Clone
        + One
        + Zero,
> MulAssign<Quaternion<T>> for Vector3<T>
{
    fn mul_assign(&mut self, rhs: Quaternion<T>) {
        *self = rhs.rotate(self.clone());
    }
}
