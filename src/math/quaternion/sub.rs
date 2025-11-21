use crate::math::Quaternion;
use std::ops::{Sub, SubAssign};

impl<T: Sub<Output = T> + Clone> Sub<T> for Quaternion<T> {
    type Output = Quaternion<T>;

    fn sub(self, rhs: T) -> Self::Output {
        Quaternion::new(
            self.x - rhs.clone(),
            self.y - rhs.clone(),
            self.z - rhs.clone(),
            self.w - rhs,
        )
    }
}

impl<T: Sub<Output = T>> Sub for Quaternion<T> {
    type Output = Quaternion<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Quaternion::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl<T: SubAssign + Clone> SubAssign<T> for Quaternion<T> {
    fn sub_assign(&mut self, rhs: T) {
        self.x -= rhs.clone();
        self.y -= rhs.clone();
        self.z -= rhs.clone();
        self.w -= rhs;
    }
}

impl<T: SubAssign> SubAssign for Quaternion<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
    }
}
