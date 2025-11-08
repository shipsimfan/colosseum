use crate::math::Vector2;
use std::ops::{Div, DivAssign};

impl<T: Div<Output = T> + Clone> Div<T> for Vector2<T> {
    type Output = Vector2<T>;

    fn div(self, rhs: T) -> Self::Output {
        Vector2::new(self.x / rhs.clone(), self.y / rhs)
    }
}

impl<T: Div<Output = T>> Div for Vector2<T> {
    type Output = Vector2<T>;

    fn div(self, rhs: Self) -> Self::Output {
        Vector2::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl<T: DivAssign + Clone> DivAssign<T> for Vector2<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs.clone();
        self.y /= rhs;
    }
}

impl<T: DivAssign> DivAssign for Vector2<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}
