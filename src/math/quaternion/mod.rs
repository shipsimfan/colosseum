mod abs;
mod add;
mod basis_vectors;
mod ceil;
mod clamp;
mod conjugate;
mod constants;
mod display;
mod div;
mod dot;
mod floor;
mod fract;
mod from;
mod index;
mod into;
mod into_f32;
mod inverse;
mod length;
mod max;
mod min;
mod mul;
mod neg;
mod new;
mod normalize;
mod rem;
mod rotate;
mod round;
mod saturate;
mod sqrt;
mod sub;

/// A representation of a rotation
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Quaternion<T> {
    /// The x imaginary part of the quaternion
    pub x: T,

    /// The y imaginary part of the quaternion
    pub y: T,

    /// The z imaginary part of the quaternion
    pub z: T,

    /// The real part of the quaternion
    pub w: T,
}

/// A quaternion made of [`f32`]s
pub type Quaternionf = Quaternion<f32>;

/// A quaternion made of [`f64`]s
pub type Quaterniond = Quaternion<f64>;

/// A quaternion made of [`u32`]s
pub type Quaternionu = Quaternion<u32>;

/// A quaternion made of [`i32`]s
pub type Quaternioni = Quaternion<i32>;
