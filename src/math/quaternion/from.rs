use crate::math::{number::Zero, Quaternion};

impl<T> From<(T, T, T, T)> for Quaternion<T> {
    fn from((x, y, z, w): (T, T, T, T)) -> Self {
        Quaternion::new(x, y, z, w)
    }
}

impl<T> From<[T; 4]> for Quaternion<T> {
    fn from(a: [T; 4]) -> Self {
        Quaternion::from_array(a)
    }
}

impl<T: Clone> From<&[T]> for Quaternion<T> {
    fn from(s: &[T]) -> Self {
        Quaternion::from_slice(s)
    }
}

impl<T: Clone> From<&[T; 3]> for Quaternion<T> {
    fn from(s: &[T; 3]) -> Self {
        Quaternion::from_slice(s)
    }
}

impl<T: Clone> From<T> for Quaternion<T> {
    fn from(v: T) -> Self {
        Quaternion::splat(v)
    }
}

impl<T: Zero> From<()> for Quaternion<T> {
    fn from(_: ()) -> Self {
        Quaternion::zero()
    }
}
