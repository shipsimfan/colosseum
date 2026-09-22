use crate::update::components::Rigidbody;
use alexandria::math::Vector3f;

impl Rigidbody {
    /// Set the mass of the rigidbody
    pub fn set_mass(&mut self, mass: f32) {
        self.mass = mass;
    }

    /// Set the velocity of the rigidbody
    pub fn set_velocity<V: Into<Vector3f>>(&mut self, velocity: V) {
        self.velocity = velocity.into();
    }

    /// Apply a force to the rigidbody for a single frame
    pub fn apply_force<V: Into<Vector3f>>(&mut self, force: V) {
        self.forces += force.into();
    }

    /// Set the current forces applied to the rigidbody
    pub fn set_forces<V: Into<Vector3f>>(&mut self, forces: V) {
        self.forces = forces.into();
    }

    /// Set whether gravity is applied to the rigidbody
    pub fn set_apply_gravity(&mut self, apply_gravity: bool) {
        self.apply_gravity = apply_gravity;
    }
}
