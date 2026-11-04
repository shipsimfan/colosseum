use crate::render::GpuDirectionalLightMatrix;
use alexandria::math::Matrix4x4f;

impl GpuDirectionalLightMatrix {
    /// Create a new [`GpuDirectionalLightMatrix`]
    pub(in crate::render::public_data::lights::directional) fn new(
        view_projection: [Matrix4x4f; 4],
    ) -> GpuDirectionalLightMatrix {
        GpuDirectionalLightMatrix { view_projection }
    }
}
