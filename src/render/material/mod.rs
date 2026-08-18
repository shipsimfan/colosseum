use alexandria::math::{Color4f, Linear};

mod id;
mod kind;
mod render;
mod shader;

mod get;
mod new;
mod set;

pub use id::*;
pub use kind::*;
pub use shader::*;

pub(crate) use render::*;

/// A material being used in rendering
///
/// This is the material as it exists in the update job
pub struct Material {
    /// The color of the material
    color: Color4f<Linear>,
}
