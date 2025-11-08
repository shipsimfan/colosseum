use crate::math::Vector2;
use std::ops::{Add, AddAssign};

impl<T: Add<Output = T> + Clone> Add<T> for Vector2<T> {
    type Output = Vector2<T>;

    fn add(self, rhs: T) -> Self::Output {
        Vector2::new(self.x + rhs.clone(), self.y + rhs)
    }
}

impl<T: Add<Output = T>> Add for Vector2<T> {
    type Output = Vector2<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: AddAssign + Clone> AddAssign<T> for Vector2<T> {
    fn add_assign(&mut self, rhs: T) {
        self.x += rhs.clone();
        self.y += rhs;
    }
}

impl<T: AddAssign> AddAssign for Vector2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
