use alexandria::math::{Color4f, Linear};
use std::sync::Arc;

mod shader;

mod bind;
mod new;
mod set;

pub use shader::*;

/// A material that is not affected by lighting
pub struct UnlitMaterial {
    /// The color of the material
    pub color: Color4f<Linear>,

    /// The shader used by the material
    pub shader: Arc<UnlitShader>,
}

/// Push constants for the unlit material shader
#[repr(C)]
pub(in crate::render::public_data::materials) struct UnlitMaterialPushConstants {
    /// The color of the material as a push constant
    color: Color4f<Linear>,

    /// The index of the object in the renderable array
    object_index: u32,
}
