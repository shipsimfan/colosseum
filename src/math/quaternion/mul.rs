use crate::math::{
    Quaternion, Vector3,
    number::{One, Zero},
};
use std::ops::{Add, Div, Mul, MulAssign, Neg, Sub};

impl<T: Mul<Output = T> + Clone> Mul<T> for Quaternion<T> {
    type Output = Quaternion<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Quaternion::new(
            self.x * rhs.clone(),
            self.y * rhs.clone(),
            self.z * rhs.clone(),
            self.w * rhs,
        )
    }
}

impl<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Clone> Mul for Quaternion<T> {
    type Output = Quaternion<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Quaternion::new(
            self.w.clone() * rhs.x.clone()
                + self.x.clone() * rhs.w.clone()
                + self.y.clone() * rhs.z.clone()
                - self.z.clone() * rhs.y.clone(),
            self.w.clone() * rhs.y.clone() - self.x.clone() * rhs.z.clone()
                + self.y.clone() * rhs.w.clone()
                + self.z.clone() * rhs.x.clone(),
            self.w.clone() * rhs.z.clone() + self.x.clone() * rhs.y.clone()
                - self.y.clone() * rhs.x.clone()
                + self.z.clone() * rhs.w.clone(),
            self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
        )
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
> Mul<Vector3<T>> for Quaternion<T>
{
    type Output = Vector3<T>;

    fn mul(self, rhs: Vector3<T>) -> Self::Output {
        self.rotate(rhs)
    }
}

impl<T: MulAssign + Clone> MulAssign<T> for Quaternion<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs.clone();
        self.y *= rhs.clone();
        self.z *= rhs.clone();
        self.w *= rhs;
    }
}

impl<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Clone> MulAssign for Quaternion<T> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone() * rhs;
    }
}
