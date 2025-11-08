use crate::math::Color3;
use std::ops::{Div, DivAssign};

impl<T: Div<Output = T> + Clone> Div<T> for Color3<T> {
    type Output = Color3<T>;

    fn div(self, rhs: T) -> Self::Output {
        Color3::new(self.r / rhs.clone(), self.g / rhs.clone(), self.b / rhs)
    }
}

impl<T: Div<Output = T>> Div for Color3<T> {
    type Output = Color3<T>;

    fn div(self, rhs: Self) -> Self::Output {
        Color3::new(self.r / rhs.r, self.g / rhs.g, self.b / rhs.b)
    }
}

impl<T: DivAssign + Clone> DivAssign<T> for Color3<T> {
    fn div_assign(&mut self, rhs: T) {
        self.r /= rhs.clone();
        self.g /= rhs.clone();
        self.b /= rhs;
    }
}

impl<T: DivAssign> DivAssign for Color3<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.r /= rhs.r;
        self.g /= rhs.g;
        self.b /= rhs.b;
    }
}
