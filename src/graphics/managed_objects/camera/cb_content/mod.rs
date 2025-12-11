use crate::math::{Matrix4x4f, Vector2f, Vector3f};

mod new;

/// The content of the camera's constant buffer
#[repr(C)]
#[derive(Clone, Copy)]
pub(in crate::graphics::managed_objects::camera) struct CameraCbContent {
    /// The view matrix of the camera
    pub view: Matrix4x4f,

    /// The position of the camera
    pub position: Vector3f,

    /// The scale the frame will be rendered down to
    pub render_scale: f32,

    /// The size of the output in pixels
    pub render_size: Vector2f,

    /// The inverse of `render_size`
    pub inverse_render_size: Vector2f,
}
