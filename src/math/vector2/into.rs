use crate::math::Vector2;

impl<T> Into<(T, T)> for Vector2<T> {
    fn into(self) -> (T, T) {
        (self.x, self.y)
    }
}

impl<T> Into<[T; 2]> for Vector2<T> {
    fn into(self) -> [T; 2] {
        [self.x, self.y]
    }
}
