use alexandria::math::{Quaternionf, Vector3f};

mod default;
mod get;
mod new;
mod set;

/// A Transform is a component that represents the position, rotation, and scale of an entity in 3D space.
#[derive(Clone)]
pub struct Transform {
    /// The position of the entity in 3D space.
    pub position: Vector3f,

    /// The rotation of the entity in 3D space, represented as a quaternion.
    pub rotation: Quaternionf,

    /// The scale of the entity in 3D space.
    pub scale: Vector3f,
}
