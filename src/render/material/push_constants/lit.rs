use alexandria::math::{Color4f, Linear};

/// The push constants provided for lit materials
#[repr(C)]
pub(crate) struct LitMaterialPushConstants {
    /// The color of the material
    pub color: Color4f<Linear>,

    /// The strength of specular highlighting
    pub specular_strength: f32,

    /// The shininess of the material
    pub shininess: f32,

    /// The index of this object's data in the renderable storage buffer
    pub object_data: u32,
}
