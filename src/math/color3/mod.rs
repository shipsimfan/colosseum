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
mod lerp;
mod max;
mod min;
mod mul;
mod neg;
mod new;
mod rem;
mod round;
mod saturate;
mod smoothstep;
mod sqrt;
mod sub;

/// A vector of size 3
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Color3<T> {
    /// The x-axis value of the vector
    pub r: T,

    /// The y-axis value of the vector
    pub g: T,

    /// The z-axis value of the vector
    pub b: T,
}

/// A vector of size 3 made of [`f32`]s
pub type Color3f = Color3<f32>;

/// A vector of size 3 made of [`f64`]s
pub type Color3d = Color3<f64>;

/// A vector of size 3 made of [`u32`]s
pub type Color3u = Color3<u32>;

/// A vector of size 3 made of [`i32`]s
pub type Color3i = Color3<i32>;
