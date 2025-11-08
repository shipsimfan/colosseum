use crate::math::Color3;

impl<T> Into<(T, T, T)> for Color3<T> {
    fn into(self) -> (T, T, T) {
        (self.r, self.g, self.b)
    }
}

impl<T> Into<[T; 3]> for Color3<T> {
    fn into(self) -> [T; 3] {
        [self.r, self.g, self.b]
    }
}
