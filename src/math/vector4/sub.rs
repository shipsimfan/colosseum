use crate::math::Vector4;
use std::ops::{Sub, SubAssign};

impl<T: Sub<Output = T> + Clone> Sub<T> for Vector4<T> {
    type Output = Vector4<T>;

    fn sub(self, rhs: T) -> Self::Output {
        Vector4::new(
            self.x - rhs.clone(),
            self.y - rhs.clone(),
            self.z - rhs.clone(),
            self.w - rhs,
        )
    }
}

impl<T: Sub<Output = T>> Sub for Vector4<T> {
    type Output = Vector4<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector4::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl<T: SubAssign + Clone> SubAssign<T> for Vector4<T> {
    fn sub_assign(&mut self, rhs: T) {
        self.x -= rhs.clone();
        self.y -= rhs.clone();
        self.z -= rhs.clone();
        self.w -= rhs;
    }
}

impl<T: SubAssign> SubAssign for Vector4<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
    }
}
