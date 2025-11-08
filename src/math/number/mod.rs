//! Traits for defining what numeric operations types can perform

mod absolute;
mod atan2;
mod ceil;
mod cos;
mod floor;
mod fract;
mod infinity;
mod into_f32;
mod max;
mod min;
mod nan;
mod neg_infinity;
mod one;
mod round;
mod sin;
mod sqrt;
mod tan;
mod zero;

pub use absolute::Absolute;
pub use atan2::Atan2;
pub use ceil::Ceil;
pub use cos::Cos;
pub use floor::Floor;
pub use fract::Fract;
pub use infinity::Infinity;
pub use into_f32::IntoF32;
pub use max::Max;
pub use min::Min;
pub use nan::NaN;
pub use neg_infinity::NegInfinity;
pub use one::One;
pub use round::Round;
pub use sin::Sin;
pub use sqrt::Sqrt;
pub use tan::Tan;
pub use zero::Zero;
