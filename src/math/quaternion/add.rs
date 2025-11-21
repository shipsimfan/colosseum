use crate::math::Quaternion;
use std::ops::{Add, AddAssign};

impl<T: Add<Output = T> + Clone> Add<T> for Quaternion<T> {
    type Output = Quaternion<T>;

    fn add(self, rhs: T) -> Self::Output {
        Quaternion::new(
            self.x + rhs.clone(),
            self.y + rhs.clone(),
            self.z + rhs.clone(),
            self.w + rhs,
        )
    }
}

impl<T: Add<Output = T>> Add for Quaternion<T> {
    type Output = Quaternion<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Quaternion::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl<T: AddAssign + Clone> AddAssign<T> for Quaternion<T> {
    fn add_assign(&mut self, rhs: T) {
        self.x += rhs.clone();
        self.y += rhs.clone();
        self.z += rhs.clone();
        self.w += rhs;
    }
}

impl<T: AddAssign> AddAssign for Quaternion<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}
