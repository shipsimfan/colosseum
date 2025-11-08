use crate::math::Vector4;
use std::ops::{Add, AddAssign};

impl<T: Add<Output = T> + Clone> Add<T> for Vector4<T> {
    type Output = Vector4<T>;

    fn add(self, rhs: T) -> Self::Output {
        Vector4::new(
            self.x + rhs.clone(),
            self.y + rhs.clone(),
            self.z + rhs.clone(),
            self.w + rhs,
        )
    }
}

impl<T: Add<Output = T>> Add for Vector4<T> {
    type Output = Vector4<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vector4::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl<T: AddAssign + Clone> AddAssign<T> for Vector4<T> {
    fn add_assign(&mut self, rhs: T) {
        self.x += rhs.clone();
        self.y += rhs.clone();
        self.z += rhs.clone();
        self.w += rhs;
    }
}

impl<T: AddAssign> AddAssign for Vector4<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}
