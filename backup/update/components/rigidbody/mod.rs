use alexandria::math::Vector3f;

mod default;
mod get;
mod new;
mod set;
mod system;

/// A rigidbody in the physics simulation
pub struct Rigidbody {
    /// The mass of the rigidbody
    mass: f32,

    /// The current velocity of the rigidbody
    velocity: Vector3f,

    /// The sum of the forces for the frame
    forces: Vector3f,

    /// Should gravity be applied to this rigidbody?
    apply_gravity: bool,
}
