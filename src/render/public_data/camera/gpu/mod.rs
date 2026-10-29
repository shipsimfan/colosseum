use alexandria::math::{Matrix4x4f, Vector3f};

/// The camera data as it exists on the GPU
#[repr(C)]
pub(in crate::render) struct GpuCamera {
    /// The camera's view-projection matrix
    view_projection: Matrix4x4f,

    /// The position of the camera in world space
    position: Vector3f,
}
