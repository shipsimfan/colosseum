use crate::math::Vector3;
use std::ops::{Rem, RemAssign};

impl<T: Rem<Output = T> + Clone> Rem<T> for Vector3<T> {
    type Output = Vector3<T>;

    fn rem(self, rhs: T) -> Self::Output {
        Vector3::new(self.x % rhs.clone(), self.y % rhs.clone(), self.z % rhs)
    }
}

impl<T: Rem<Output = T>> Rem for Vector3<T> {
    type Output = Vector3<T>;

    fn rem(self, rhs: Self) -> Self::Output {
        Vector3::new(self.x % rhs.x, self.y % rhs.y, self.z % rhs.z)
    }
}

impl<T: RemAssign + Clone> RemAssign<T> for Vector3<T> {
    fn rem_assign(&mut self, rhs: T) {
        self.x %= rhs.clone();
        self.y %= rhs.clone();
        self.z %= rhs;
    }
}

impl<T: RemAssign> RemAssign for Vector3<T> {
    fn rem_assign(&mut self, rhs: Self) {
        self.x %= rhs.x;
        self.y %= rhs.y;
        self.z %= rhs.z;
    }
}
