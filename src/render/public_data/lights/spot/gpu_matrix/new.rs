use crate::render::GpuSpotLightMatrix;
use alexandria::math::Matrix4x4f;

impl GpuSpotLightMatrix {
    /// Create a new [`GpuSpotLightMatrix`]
    pub(in crate::render::public_data::lights::spot) fn new(
        view_projection: Matrix4x4f,
    ) -> GpuSpotLightMatrix {
        GpuSpotLightMatrix { view_projection }
    }
}
