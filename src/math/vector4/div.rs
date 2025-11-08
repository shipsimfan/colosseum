use crate::math::Vector4;
use std::ops::{Div, DivAssign};

impl<T: Div<Output = T> + Clone> Div<T> for Vector4<T> {
    type Output = Vector4<T>;

    fn div(self, rhs: T) -> Self::Output {
        Vector4::new(
            self.x / rhs.clone(),
            self.y / rhs.clone(),
            self.z / rhs.clone(),
            self.w / rhs,
        )
    }
}

impl<T: Div<Output = T>> Div for Vector4<T> {
    type Output = Vector4<T>;

    fn div(self, rhs: Self) -> Self::Output {
        Vector4::new(
            self.x / rhs.x,
            self.y / rhs.y,
            self.z / rhs.z,
            self.w / rhs.w,
        )
    }
}

impl<T: DivAssign + Clone> DivAssign<T> for Vector4<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs.clone();
        self.y /= rhs.clone();
        self.z /= rhs.clone();
        self.w /= rhs;
    }
}

impl<T: DivAssign> DivAssign for Vector4<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
        self.w /= rhs.w;
    }
}
