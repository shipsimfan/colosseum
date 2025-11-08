use crate::math::Vector3;
use std::ops::{Div, DivAssign};

impl<T: Div<Output = T> + Clone> Div<T> for Vector3<T> {
    type Output = Vector3<T>;

    fn div(self, rhs: T) -> Self::Output {
        Vector3::new(self.x / rhs.clone(), self.y / rhs.clone(), self.z / rhs)
    }
}

impl<T: Div<Output = T>> Div for Vector3<T> {
    type Output = Vector3<T>;

    fn div(self, rhs: Self) -> Self::Output {
        Vector3::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl<T: DivAssign + Clone> DivAssign<T> for Vector3<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs.clone();
        self.y /= rhs.clone();
        self.z /= rhs;
    }
}

impl<T: DivAssign> DivAssign for Vector3<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}
