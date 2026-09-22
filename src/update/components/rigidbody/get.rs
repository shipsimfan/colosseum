use crate::update::components::Rigidbody;
use alexandria::math::Vector3f;

impl Rigidbody {
    /// Get the mass of the rigidbody
    pub fn mass(&self) -> f32 {
        self.mass
    }

    /// Get a mutable reference to the mass of the rigidbody
    pub fn mass_mut(&mut self) -> &mut f32 {
        &mut self.mass
    }

    /// Get the velocity of the rigidbody
    pub fn velocity(&self) -> Vector3f {
        self.velocity
    }

    /// Get a mutable reference to the velocity of the rigidbody
    pub fn velocity_mut(&mut self) -> &mut Vector3f {
        &mut self.velocity
    }

    /// Get the current forces applied to the rigidbody
    pub fn forces(&self) -> Vector3f {
        self.forces
    }

    /// Get a mutable reference to the current forces applied to the rigidbody
    pub fn forces_mut(&mut self) -> &mut Vector3f {
        &mut self.forces
    }

    /// Is gravity applied to the rigidbody?    
    pub fn apply_gravity(&self) -> bool {
        self.apply_gravity
    }
}
