use alexandria::math::{Quaternionf, Vector3f};

mod gpu;
mod projection;

mod create_buffer;
mod default;
mod new;
mod system;

pub use projection::*;

pub(in crate::render) use gpu::*;

/// The currently active camera in the scene
pub struct Camera {
    /// The position of the camera in the scene
    pub position: Vector3f,

    /// The rotation of the camera in the scene
    pub rotation: Quaternionf,

    /// The projection used by the camera
    pub projection: CameraProjection,
}
