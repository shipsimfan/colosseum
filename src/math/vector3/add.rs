use crate::math::Vector3;
use std::ops::{Add, AddAssign};

impl<T: Add<Output = T> + Clone> Add<T> for Vector3<T> {
    type Output = Vector3<T>;

    fn add(self, rhs: T) -> Self::Output {
        Vector3::new(self.x + rhs.clone(), self.y + rhs.clone(), self.z + rhs)
    }
}

impl<T: Add<Output = T>> Add for Vector3<T> {
    type Output = Vector3<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T: AddAssign + Clone> AddAssign<T> for Vector3<T> {
    fn add_assign(&mut self, rhs: T) {
        self.x += rhs.clone();
        self.y += rhs.clone();
        self.z += rhs;
    }
}

impl<T: AddAssign> AddAssign for Vector3<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
