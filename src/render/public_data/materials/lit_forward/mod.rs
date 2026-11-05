use alexandria::math::{Color3f, Color4f, Linear};
use std::sync::Arc;

mod materials;
mod shader;

mod bind;
mod new;
mod set;

pub use shader::*;

pub(in crate::render::public_data::materials) use materials::*;

/// A material being used in lit forward rendering
pub struct LitForwardMaterial {
    /// The color of the material
    pub color: Color3f<Linear>,

    /// The strength of specular reflections
    pub specular_strength: f32,

    /// The shininess of the material
    pub shininess: f32,

    /// The shader used by the material
    pub shader: Arc<LitForwardShader>,
}

/// The push constants provided for lit materials
#[repr(C)]
pub(in crate::render::public_data::materials) struct LitForwardMaterialPushConstants {
    /// The color of the material
    color: Color4f<Linear>,

    /// The strength of specular highlighting
    specular_strength: f32,

    /// The shininess of the material
    shininess: f32,

    /// The index of this object's data in the renderable storage buffer
    object_index: u32,
}
