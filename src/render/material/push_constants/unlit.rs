use alexandria::{
    gpu::GpuAddress,
    math::{Color4f, Linear},
};

use crate::render::ObjectData;

/// The push constants provided for unlit materials
#[repr(C)]
pub(crate) struct UnlitMaterialPushConstants {
    /// The color of the material
    pub color: Color4f<Linear>,

    /// A pointer to the object data
    pub object_data: GpuAddress<ObjectData>,
}
