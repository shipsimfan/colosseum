/// A representation of a rotation
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Quaternion<T> {
    /// The real part of the quaternion
    pub w: T,

    /// The x imaginary part of the quaternion
    pub x: T,

    /// The y imaginary part of the quaternion
    pub y: T,

    /// The z imaginary part of the quaternion
    pub z: T,
}

/// A quaternion made of [`f32`]s
pub type Quaternionf = Quaternion<f32>;

/// A quaternion made of [`f64`]s
pub type Quaterniond = Quaternion<f64>;

/// A quaternion made of [`u32`]s
pub type Quaternionu = Quaternion<u32>;

/// A quaternion made of [`i32`]s
pub type Quaternioni = Quaternion<i32>;
