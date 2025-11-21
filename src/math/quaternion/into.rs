use crate::math::Quaternion;

impl<T> Into<(T, T, T, T)> for Quaternion<T> {
    fn into(self) -> (T, T, T, T) {
        (self.x, self.y, self.z, self.w)
    }
}

impl<T> Into<[T; 4]> for Quaternion<T> {
    fn into(self) -> [T; 4] {
        [self.x, self.y, self.z, self.w]
    }
}
