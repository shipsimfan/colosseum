use crate::render::GpuPointLightMatrix;
use alexandria::math::Matrix4x4f;

impl GpuPointLightMatrix {
    /// Create a new [`GpuPointLightMatrix`]
    pub(in crate::render::public_data::lights::point) fn new(
        view_projection: [Matrix4x4f; 6],
    ) -> GpuPointLightMatrix {
        GpuPointLightMatrix { view_projection }
    }
}
