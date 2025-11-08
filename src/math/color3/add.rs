use crate::math::Color3;
use std::ops::{Add, AddAssign};

impl<T: Add<Output = T> + Clone> Add<T> for Color3<T> {
    type Output = Color3<T>;

    fn add(self, rhs: T) -> Self::Output {
        Color3::new(self.r + rhs.clone(), self.g + rhs.clone(), self.b + rhs)
    }
}

impl<T: Add<Output = T>> Add for Color3<T> {
    type Output = Color3<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Color3::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl<T: AddAssign + Clone> AddAssign<T> for Color3<T> {
    fn add_assign(&mut self, rhs: T) {
        self.r += rhs.clone();
        self.g += rhs.clone();
        self.b += rhs;
    }
}

impl<T: AddAssign> AddAssign for Color3<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}
