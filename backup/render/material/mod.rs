use alexandria::math::{Color4f, Srgb};

mod id;
mod kind;
mod push_constants;
mod render;
mod shader;

mod get;
mod new;
mod set;

pub use id::*;
pub use kind::*;
pub use shader::*;

pub(crate) use push_constants::*;
pub(crate) use render::*;

/// A material being used in rendering
///
/// This is the material as it exists in the update job
pub struct Material {
    /// The color of the material
    color: Color4f<Srgb>,

    /// The strength of specular reflections
    specular_strength: f32,

    /// The shininess of the material
    shininess: f32,
}
