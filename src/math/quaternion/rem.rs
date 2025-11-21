use crate::math::Quaternion;
use std::ops::{Rem, RemAssign};

impl<T: Rem<Output = T> + Clone> Rem<T> for Quaternion<T> {
    type Output = Quaternion<T>;

    fn rem(self, rhs: T) -> Self::Output {
        Quaternion::new(
            self.x % rhs.clone(),
            self.y % rhs.clone(),
            self.z % rhs.clone(),
            self.w % rhs,
        )
    }
}

impl<T: Rem<Output = T>> Rem for Quaternion<T> {
    type Output = Quaternion<T>;

    fn rem(self, rhs: Self) -> Self::Output {
        Quaternion::new(
            self.x % rhs.x,
            self.y % rhs.y,
            self.z % rhs.z,
            self.w % rhs.w,
        )
    }
}

impl<T: RemAssign + Clone> RemAssign<T> for Quaternion<T> {
    fn rem_assign(&mut self, rhs: T) {
        self.x %= rhs.clone();
        self.y %= rhs.clone();
        self.z %= rhs.clone();
        self.w %= rhs;
    }
}

impl<T: RemAssign> RemAssign for Quaternion<T> {
    fn rem_assign(&mut self, rhs: Self) {
        self.x %= rhs.x;
        self.y %= rhs.y;
        self.z %= rhs.z;
        self.w %= rhs.w;
    }
}
