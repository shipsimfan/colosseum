use crate::math::{number::Zero, Vector3};

impl<T> From<(T, T, T)> for Vector3<T> {
    fn from((x, y, z): (T, T, T)) -> Self {
        Vector3::new(x, y, z)
    }
}

impl<T> From<[T; 3]> for Vector3<T> {
    fn from(a: [T; 3]) -> Self {
        Vector3::from_array(a)
    }
}

impl<T: Clone> From<&[T]> for Vector3<T> {
    fn from(s: &[T]) -> Self {
        Vector3::from_slice(s)
    }
}

impl<T: Clone> From<&[T; 3]> for Vector3<T> {
    fn from(s: &[T; 3]) -> Self {
        Vector3::from_slice(s)
    }
}

impl<T: Clone> From<T> for Vector3<T> {
    fn from(v: T) -> Self {
        Vector3::splat(v)
    }
}

impl<T: Zero> From<()> for Vector3<T> {
    fn from(_: ()) -> Self {
        Vector3::zero()
    }
}
