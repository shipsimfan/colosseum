use crate::render::GpuDirectionalLight;
use alexandria::math::{Color4f, Linear, Vector3f};

impl GpuDirectionalLight {
    /// Create a new [`GpuDirectionalLight`]
    pub(in crate::render::public_data::lights::directional) fn new(
        color: Color4f<Linear>,
        direction: Vector3f,
        cascade_far_depths: [f32; 4],
    ) -> GpuDirectionalLight {
        GpuDirectionalLight {
            color,
            direction,
            reserved: 0,
            cascade_far_depths,
        }
    }
}
