use crate::math::Color3f;

/// The content of the light constant buffer
#[repr(C)]
#[derive(Clone, Copy)]
pub(in crate::graphics::context::managed_objects::lights::constant_buffer) struct LightCbContent {
    /// The color of the ambient light
    pub ambient_color: Color3f,

    /// The intensity of the ambient light
    pub ambient_intensity: f32,

    /// The current number of directional lights
    pub num_directional_lights: u32,

    /// The current number of point lights
    pub num_point_lights: u32,

    /// Reserved values to pad to a multiple of 16-bytes
    pub reserved: [u32; 2],
}

impl Default for LightCbContent {
    fn default() -> Self {
        LightCbContent {
            ambient_color: Color3f::WHITE,
            ambient_intensity: 0.1,
            num_directional_lights: 0,
            num_point_lights: 0,
            reserved: [0; 2],
        }
    }
}
