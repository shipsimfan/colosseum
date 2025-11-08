use crate::math::Vector3;
use std::ops::{Sub, SubAssign};

impl<T: Sub<Output = T> + Clone> Sub<T> for Vector3<T> {
    type Output = Vector3<T>;

    fn sub(self, rhs: T) -> Self::Output {
        Vector3::new(self.x - rhs.clone(), self.y - rhs.clone(), self.z - rhs)
    }
}

impl<T: Sub<Output = T>> Sub for Vector3<T> {
    type Output = Vector3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T: SubAssign + Clone> SubAssign<T> for Vector3<T> {
    fn sub_assign(&mut self, rhs: T) {
        self.x -= rhs.clone();
        self.y -= rhs.clone();
        self.z -= rhs;
    }
}

impl<T: SubAssign> SubAssign for Vector3<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}
