use crate::update::components::Rigidbody;
use alexandria::math::Vector3f;

impl Rigidbody {
    /// Create a new [`Rigidbody`] component
    pub fn new(mass: f32) -> Rigidbody {
        Rigidbody {
            mass,
            velocity: Vector3f::ZERO,
            forces: Vector3f::ZERO,
            apply_gravity: true,
        }
    }
}
