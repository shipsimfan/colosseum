use crate::render::LightMetadata;
use alexandria::math::Color4f;

impl Default for LightMetadata {
    fn default() -> Self {
        LightMetadata {
            ambient_light: Color4f::WHITE,
            num_directional_lights: 0,
            num_point_lights: 0,
            num_spot_lights: 0,
        }
    }
}
