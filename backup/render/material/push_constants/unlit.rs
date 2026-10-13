use alexandria::math::{Color4f, Linear};

/// The push constants provided for unlit materials
#[repr(C)]
pub(crate) struct UnlitMaterialPushConstants {
    /// The color of the material
    pub color: Color4f<Linear>,

    /// The index of this object's data in the renderable storage buffer
    pub object_data: u32,
}
