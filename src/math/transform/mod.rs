use crate::math::{Matrix4x4f, Vector3f};

mod as_ref;
mod matrix;
mod new;
mod position;
mod rotation;
mod scale;
mod update;

/// The state of a 3d object
pub struct Transform {
    /// The current position
    position: Vector3f,

    /// The current rotation represented in euler angles
    ///
    /// TODO: Change this to a quaternion
    rotation: Vector3f,

    /// The scaling factor to be applied
    scale: Vector3f,

    /// The combined matrix representing the results
    matrix: Matrix4x4f,

    /// Has the transform changed this frame?
    dirty: bool,
}
