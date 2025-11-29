use crate::math::Color3f;

/// The content of the light constant buffer
#[repr(C)]
#[derive(Clone, Copy)]
pub(in crate::graphics::context::managed_objects::lights::constant_buffer) struct LightCbContent {
    /// The color of the ambient light
    pub ambient_color: Color3f,

    /// The intensity of the ambient light
    pub ambient_intensity: f32,
}

impl Default for LightCbContent {
    fn default() -> Self {
        LightCbContent {
            ambient_color: Color3f::WHITE,
            ambient_intensity: 0.1,
        }
    }
}
