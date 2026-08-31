use alexandria::math::{Color4f, Linear};

/// Metadata about all lights
#[repr(C)]
pub(in crate::render::data::lighting) struct LightingMetadata {
    /// The color of the ambient light
    pub ambient_light: Color4f<Linear>,

    /// The number of directional lights in the scene
    pub num_directional_lights: u32,

    /// The number of point lights in the scene
    pub num_point_lights: u32,

    /// The number of spot lights in the scene
    pub num_spot_lights: u32,
}

impl Default for LightingMetadata {
    fn default() -> Self {
        LightingMetadata {
            ambient_light: Color4f::WHITE,
            num_directional_lights: 0,
            num_point_lights: 0,
            num_spot_lights: 0,
        }
    }
}
