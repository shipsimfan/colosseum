use crate::render::GpuSpotLight;
use alexandria::math::{Color4f, Linear, Vector3f};

impl GpuSpotLight {
    /// Create a new [`GpuSpotLight`]
    pub(in crate::render::public_data::lights::spot) fn new(
        color: Color4f<Linear>,
        position: Vector3f,
        range: f32,
        direction: Vector3f,
        cutoff_angle: f32,
        falloff_angle: f32,
    ) -> GpuSpotLight {
        GpuSpotLight {
            color,
            position,
            range,
            direction,
            cutoff_angle,
            falloff_angle,
        }
    }
}
