use crate::math::Quaternion;
use std::ops::{Div, DivAssign};

impl<T: Div<Output = T> + Clone> Div<T> for Quaternion<T> {
    type Output = Quaternion<T>;

    fn div(self, rhs: T) -> Self::Output {
        Quaternion::new(
            self.x / rhs.clone(),
            self.y / rhs.clone(),
            self.z / rhs.clone(),
            self.w / rhs,
        )
    }
}

impl<T: Div<Output = T>> Div for Quaternion<T> {
    type Output = Quaternion<T>;

    fn div(self, rhs: Self) -> Self::Output {
        Quaternion::new(
            self.x / rhs.x,
            self.y / rhs.y,
            self.z / rhs.z,
            self.w / rhs.w,
        )
    }
}

impl<T: DivAssign + Clone> DivAssign<T> for Quaternion<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs.clone();
        self.y /= rhs.clone();
        self.z /= rhs.clone();
        self.w /= rhs;
    }
}

impl<T: DivAssign> DivAssign for Quaternion<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
        self.w /= rhs.w;
    }
}
