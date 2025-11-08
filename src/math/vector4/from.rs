use crate::math::{number::Zero, Vector4};

impl<T> From<(T, T, T, T)> for Vector4<T> {
    fn from((x, y, z, w): (T, T, T, T)) -> Self {
        Vector4::new(x, y, z, w)
    }
}

impl<T> From<[T; 4]> for Vector4<T> {
    fn from(a: [T; 4]) -> Self {
        Vector4::from_array(a)
    }
}

impl<T: Clone> From<&[T]> for Vector4<T> {
    fn from(s: &[T]) -> Self {
        Vector4::from_slice(s)
    }
}

impl<T: Clone> From<&[T; 3]> for Vector4<T> {
    fn from(s: &[T; 3]) -> Self {
        Vector4::from_slice(s)
    }
}

impl<T: Clone> From<T> for Vector4<T> {
    fn from(v: T) -> Self {
        Vector4::splat(v)
    }
}

impl<T: Zero> From<()> for Vector4<T> {
    fn from(_: ()) -> Self {
        Vector4::zero()
    }
}
