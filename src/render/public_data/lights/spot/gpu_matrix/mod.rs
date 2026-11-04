use alexandria::math::Matrix4x4f;

mod new;

/// The GPU representation of a spot light's view-projection matrix
#[repr(C, align(16))]
pub(in crate::render) struct GpuSpotLightMatrix {
    /// The spot light's view-projection matrix
    view_projection: Matrix4x4f,
}
