use crate::math::{number::Zero, Color3};

impl<T> From<(T, T, T)> for Color3<T> {
    fn from((r, g, b): (T, T, T)) -> Self {
        Color3::new(r, g, b)
    }
}

impl<T> From<[T; 3]> for Color3<T> {
    fn from(a: [T; 3]) -> Self {
        Color3::from_array(a)
    }
}

impl<T: Clone> From<&[T]> for Color3<T> {
    fn from(s: &[T]) -> Self {
        Color3::from_slice(s)
    }
}

impl<T: Clone> From<&[T; 3]> for Color3<T> {
    fn from(s: &[T; 3]) -> Self {
        Color3::from_slice(s)
    }
}

impl<T: Clone> From<T> for Color3<T> {
    fn from(v: T) -> Self {
        Color3::splat(v)
    }
}

impl<T: Zero> From<()> for Color3<T> {
    fn from(_: ()) -> Self {
        Color3::black()
    }
}
