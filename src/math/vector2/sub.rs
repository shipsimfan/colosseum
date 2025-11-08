use crate::math::Vector2;
use std::ops::{Sub, SubAssign};

impl<T: Sub<Output = T> + Clone> Sub<T> for Vector2<T> {
    type Output = Vector2<T>;

    fn sub(self, rhs: T) -> Self::Output {
        Vector2::new(self.x - rhs.clone(), self.y - rhs)
    }
}

impl<T: Sub<Output = T>> Sub for Vector2<T> {
    type Output = Vector2<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: SubAssign + Clone> SubAssign<T> for Vector2<T> {
    fn sub_assign(&mut self, rhs: T) {
        self.x -= rhs.clone();
        self.y -= rhs;
    }
}

impl<T: SubAssign> SubAssign for Vector2<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
