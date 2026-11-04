use alexandria::math::{Color4f, Linear};

mod create_buffer;
mod default;
mod system;

/// Metadata about all lights
#[repr(C)]
#[derive(Clone)]
pub(in crate::render) struct LightMetadata {
    /// The color of the ambient light
    ambient_light: Color4f<Linear>,

    /// The number of directional lights in the scene
    num_directional_lights: u32,

    /// The number of point lights in the scene
    num_point_lights: u32,

    /// The number of spot lights in the scene
    num_spot_lights: u32,
}
