use crate::{
    math::{Matrix4x4f, Quaternionf, Vector3f},
    util::Handle,
};

mod transforms;

mod as_ref;
mod get;
mod look_at;
mod new;
mod set;
mod update;

pub use transforms::Transforms;

/// A handle to a [`Transform`]
pub type TransformHandle = Handle<Transform>;

/// The state of a 3d object
pub struct Transform {
    /// The current position
    position: Vector3f,

    /// The current rotation represented in euler angles
    rotation: Quaternionf,

    /// The scaling factor to be applied
    scale: Vector3f,

    /// The combined matrix representing the results
    matrix: Matrix4x4f,

    /// Has the transform changed this frame?
    dirty: bool,

    /// Monotonically increasing counter tracking how many times this transform's matrix has been
    /// updated.
    ///
    /// Used by multiple consumers to detect when a cached transform matrix is out of date.
    epoch: u32,
}
