use crate::math::{
    Matrix4x4, Vector4,
    number::{One, Zero},
};

impl<T> From<[[T; 4]; 4]> for Matrix4x4<T> {
    fn from(a: [[T; 4]; 4]) -> Self {
        Matrix4x4::new_row(a)
    }
}

impl<T: Clone> From<&[&[T]]> for Matrix4x4<T> {
    fn from(a: &[&[T]]) -> Self {
        Matrix4x4::from_row_slices(a)
    }
}

impl<T: Clone> From<&[&[T; 4]; 4]> for Matrix4x4<T> {
    fn from(a: &[&[T; 4]; 4]) -> Self {
        Matrix4x4::from_row_slices(&[a[0], a[1], a[2], a[3]])
    }
}

impl<T> From<[T; 16]> for Matrix4x4<T> {
    fn from(a: [T; 16]) -> Self {
        Matrix4x4::from_row_array(a)
    }
}

impl<T: Clone> From<&[T]> for Matrix4x4<T> {
    fn from(s: &[T]) -> Self {
        Matrix4x4::from_row_slice(s)
    }
}

impl<T: Clone> From<&[T; 16]> for Matrix4x4<T> {
    fn from(s: &[T; 16]) -> Self {
        Matrix4x4::from_row_slice(s)
    }
}

impl<T> From<[Vector4<T>; 4]> for Matrix4x4<T> {
    fn from(a: [Vector4<T>; 4]) -> Self {
        Matrix4x4::from_row_vec_array(a)
    }
}

impl<T: Clone> From<&[Vector4<T>]> for Matrix4x4<T> {
    fn from(a: &[Vector4<T>]) -> Self {
        Matrix4x4::from_row_vec_slice(a)
    }
}

impl<T: Clone> From<&[Vector4<T>; 4]> for Matrix4x4<T> {
    fn from(a: &[Vector4<T>; 4]) -> Self {
        Matrix4x4::from_row_vec_slice(a)
    }
}

impl<T: Clone> From<T> for Matrix4x4<T> {
    fn from(v: T) -> Self {
        Matrix4x4::splat(v)
    }
}

impl<T: Zero + One> From<()> for Matrix4x4<T> {
    fn from(_: ()) -> Self {
        Matrix4x4::identity()
    }
}
