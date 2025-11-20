mod abs;
mod add;
mod ceil;
mod clamp;
mod constants;
mod display;
mod div;
mod floor;
mod fract;
mod from;
mod index;
mod into;
mod into_f32;
mod max;
mod min;
mod mul;
mod neg;
mod new;
mod rem;
mod round;
mod saturate;
mod sqrt;
mod sub;
mod transpose;

/// A matrix with 4 columns and 4 rows
///
/// The matrix is stored in row major
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Matrix4x4<T> {
    /// The values that make up the matrix, in row major
    pub v: [[T; 4]; 4],
}

/// A matrix with 4 columns and 4 rows made of [`f32`]s
pub type Matrix4x4f = Matrix4x4<f32>;

/// A matrix with 4 columns and 4 rows made of [`f64`]s
pub type Matrix4x4d = Matrix4x4<f64>;

/// A matrix with 4 columns and 4 rows made of [`u32`]s
pub type Matrix4x4u = Matrix4x4<u32>;

/// A matrix with 4 columns and 4 rows made of [`i32`]s
pub type Matrix4x4i = Matrix4x4<i32>;
