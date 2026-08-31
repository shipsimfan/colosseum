use alexandria::math::{Matrix4x4f, Vector3f};

mod new;
mod set;

/// The data that is passed to shader programs describing the camera's view of the scene
#[repr(C)]
pub(in crate::render::data) struct CameraRenderData {
    /// The camera's view-projection matrix
    view_projection: Matrix4x4f,

    /// The position of the camera in world space
    position: Vector3f,
}
