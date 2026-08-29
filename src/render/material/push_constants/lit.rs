use alexandria::{
    gpu::GpuAddress,
    math::{Color4f, Linear},
};

use crate::render::ObjectData;

/// The push constants provided for lit materials
#[repr(C)]
pub(crate) struct LitMaterialPushConstants {
    /// The color of the material
    pub color: Color4f<Linear>,

    /// The strength of specular highlighting
    pub specular_strength: f32,

    /// The shininess of the material
    pub shininess: f32,

    /// A pointer to the object data
    pub object_data: GpuAddress<ObjectData>,
}
