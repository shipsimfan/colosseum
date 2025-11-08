use crate::math::Vector2;
use std::ops::{Rem, RemAssign};

impl<T: Rem<Output = T> + Clone> Rem<T> for Vector2<T> {
    type Output = Vector2<T>;

    fn rem(self, rhs: T) -> Self::Output {
        Vector2::new(self.x % rhs.clone(), self.y % rhs)
    }
}

impl<T: Rem<Output = T>> Rem for Vector2<T> {
    type Output = Vector2<T>;

    fn rem(self, rhs: Self) -> Self::Output {
        Vector2::new(self.x % rhs.x, self.y % rhs.y)
    }
}

impl<T: RemAssign + Clone> RemAssign<T> for Vector2<T> {
    fn rem_assign(&mut self, rhs: T) {
        self.x %= rhs.clone();
        self.y %= rhs;
    }
}

impl<T: RemAssign> RemAssign for Vector2<T> {
    fn rem_assign(&mut self, rhs: Self) {
        self.x %= rhs.x;
        self.y %= rhs.y;
    }
}
