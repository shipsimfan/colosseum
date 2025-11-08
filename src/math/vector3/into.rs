use crate::math::Vector3;

impl<T> Into<(T, T, T)> for Vector3<T> {
    fn into(self) -> (T, T, T) {
        (self.x, self.y, self.z)
    }
}

impl<T> Into<[T; 3]> for Vector3<T> {
    fn into(self) -> [T; 3] {
        [self.x, self.y, self.z]
    }
}
