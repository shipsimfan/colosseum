mod abs;
mod add;
mod angle_between;
mod ceil;
mod clamp;
mod constants;
mod display;
mod distance;
mod div;
mod dot;
mod floor;
mod fract;
mod from;
mod index;
mod into;
mod into_f32;
mod length;
mod lerp;
mod max;
mod min;
mod mul;
mod neg;
mod new;
mod nlerp;
mod normalize;
mod rem;
mod round;
mod saturate;
mod slerp;
mod smoothstep;
mod sqrt;
mod sub;
mod swizzle;

/// A vector of size 3
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector4<T> {
    /// The x-axis value of the vector
    pub x: T,

    /// The y-axis value of the vector
    pub y: T,

    /// The z-axis value of the vector
    pub z: T,

    /// The w-axis value of the vector
    pub w: T,
}

/// A vector of size 3 made of [`f32`]s
pub type Vector4f = Vector4<f32>;

/// A vector of size 3 made of [`f64`]s
pub type Vector4d = Vector4<f64>;

/// A vector of size 3 made of [`u32`]s
pub type Vector4u = Vector4<u32>;

/// A vector of size 3 made of [`i32`]s
pub type Vector4i = Vector4<i32>;
