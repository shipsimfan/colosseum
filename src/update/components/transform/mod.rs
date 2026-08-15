use alexandria::math::{Matrix4x4f, Quaternionf, Vector3f};

mod default;
mod get;
mod new;
mod set;

/// A Transform is a component that represents the position, rotation, and scale of an entity in 3D space.
pub struct Transform {
    /// The position of the entity in 3D space.
    position: Vector3f,

    /// The rotation of the entity in 3D space, represented as a quaternion.
    rotation: Quaternionf,

    /// The scale of the entity in 3D space.
    scale: Vector3f,

    /// Has the transform been modified since the last update?
    dirty: bool,

    /// Is the matrix a camera matrix?
    camera: bool,

    /// The matrix representing this transform
    matrix: Matrix4x4f,
}
