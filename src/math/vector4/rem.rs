use crate::math::Vector4;
use std::ops::{Rem, RemAssign};

impl<T: Rem<Output = T> + Clone> Rem<T> for Vector4<T> {
    type Output = Vector4<T>;

    fn rem(self, rhs: T) -> Self::Output {
        Vector4::new(
            self.x % rhs.clone(),
            self.y % rhs.clone(),
            self.z % rhs.clone(),
            self.w % rhs,
        )
    }
}

impl<T: Rem<Output = T>> Rem for Vector4<T> {
    type Output = Vector4<T>;

    fn rem(self, rhs: Self) -> Self::Output {
        Vector4::new(
            self.x % rhs.x,
            self.y % rhs.y,
            self.z % rhs.z,
            self.w % rhs.w,
        )
    }
}

impl<T: RemAssign + Clone> RemAssign<T> for Vector4<T> {
    fn rem_assign(&mut self, rhs: T) {
        self.x %= rhs.clone();
        self.y %= rhs.clone();
        self.z %= rhs.clone();
        self.w %= rhs;
    }
}

impl<T: RemAssign> RemAssign for Vector4<T> {
    fn rem_assign(&mut self, rhs: Self) {
        self.x %= rhs.x;
        self.y %= rhs.y;
        self.z %= rhs.z;
        self.w %= rhs.w;
    }
}
