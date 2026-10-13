use crate::render::{MaterialKind, Shader};
use alexandria::{
    gpu::VulkanPipeline,
    math::{Color4f, Linear},
};
use std::sync::Arc;

mod bind;
mod new;
mod set;

/// A material being used in rendering
///
/// This is the material as it exists in the render job
pub(crate) struct RenderMaterial {
    /// The kind of material this is
    kind: MaterialKind,

    /// The color of the material
    color: Color4f<Linear>,

    /// The strength of specular reflection
    specular_strength: f32,

    /// The shininess of the material
    shininess: f32,

    /// The pipeline this material is using
    pipeline: VulkanPipeline,

    /// The shader being used by this material
    #[allow(unused)]
    shader: Arc<Shader>,
}
