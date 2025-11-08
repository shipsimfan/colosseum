use crate::math::Color3;
use std::ops::{Sub, SubAssign};

impl<T: Sub<Output = T> + Clone> Sub<T> for Color3<T> {
    type Output = Color3<T>;

    fn sub(self, rhs: T) -> Self::Output {
        Color3::new(self.r - rhs.clone(), self.g - rhs.clone(), self.b - rhs)
    }
}

impl<T: Sub<Output = T>> Sub for Color3<T> {
    type Output = Color3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Color3::new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
    }
}

impl<T: SubAssign + Clone> SubAssign<T> for Color3<T> {
    fn sub_assign(&mut self, rhs: T) {
        self.r -= rhs.clone();
        self.g -= rhs.clone();
        self.b -= rhs;
    }
}

impl<T: SubAssign> SubAssign for Color3<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
    }
}
