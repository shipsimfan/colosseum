use crate::render::GpuPointLight;
use alexandria::math::{Color4f, Linear, Vector3f};

impl GpuPointLight {
    /// Create a new [`GpuPointLight`] instance
    pub(in crate::render::public_data::lights::point) fn new(
        color: Color4f<Linear>,
        position: Vector3f,
        range: f32,
    ) -> GpuPointLight {
        GpuPointLight {
            color,
            position,
            range,
        }
    }
}
