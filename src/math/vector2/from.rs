use crate::math::{number::Zero, Vector2};

impl<T> From<(T, T)> for Vector2<T> {
    fn from((x, y): (T, T)) -> Self {
        Vector2::new(x, y)
    }
}

impl<T> From<[T; 2]> for Vector2<T> {
    fn from(a: [T; 2]) -> Self {
        Vector2::from_array(a)
    }
}

impl<T: Clone> From<&[T]> for Vector2<T> {
    fn from(s: &[T]) -> Self {
        Vector2::from_slice(s)
    }
}

impl<T: Clone> From<&[T; 2]> for Vector2<T> {
    fn from(s: &[T; 2]) -> Self {
        Vector2::from_slice(s)
    }
}

impl<T: Clone> From<T> for Vector2<T> {
    fn from(v: T) -> Self {
        Vector2::splat(v)
    }
}

impl<T: Zero> From<()> for Vector2<T> {
    fn from(_: ()) -> Self {
        Vector2::zero()
    }
}
