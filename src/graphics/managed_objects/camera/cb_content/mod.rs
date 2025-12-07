use crate::math::{Matrix4x4f, Vector3f};

mod new;

/// The content of the camera's constant buffer
#[repr(C)]
#[derive(Clone, Copy)]
pub(in crate::graphics::managed_objects::camera) struct CameraCbContent {
    /// The view matrix of the camera
    pub view: Matrix4x4f,

    /// The position of the camera
    pub position: Vector3f,

    /// A reserved value to pad it
    reserved: f32,
}
