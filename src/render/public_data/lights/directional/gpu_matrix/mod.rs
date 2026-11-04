use alexandria::math::Matrix4x4f;

mod new;

/// The GPU representation of a directional light's view-projection matrix
#[repr(C, align(16))]
pub(in crate::render) struct GpuDirectionalLightMatrix {
    /// The directional light's view-projection matrices
    view_projection: [Matrix4x4f; 4],
}
