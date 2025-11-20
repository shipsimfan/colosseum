//! Graphics focused math utilities

pub mod number;

mod color3;
mod matrix4x4;
mod quaternion;
mod rational;
mod transform;
mod vector2;
mod vector3;
mod vector4;

pub use color3::*;
pub use matrix4x4::*;
pub use quaternion::*;
pub use rational::*;
pub use transform::Transform;
pub use vector2::*;
pub use vector3::*;
pub use vector4::*;
