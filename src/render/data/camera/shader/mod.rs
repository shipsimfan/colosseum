use alexandria::math::Matrix4x4f;

mod new;
mod set;

/// The data that is passed to shader programs describing the camera's view of the scene
#[repr(C)]
pub(in crate::render::data::camera) struct CameraShaderData {
    /// The camera's view-projection matrix
    view_projection: Matrix4x4f,
}
