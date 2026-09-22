use crate::update::PhysicsData;
use alexandria::math::Vector3f;

impl PhysicsData {
    /// Get the current force of gravity
    pub fn gravity(&self) -> Vector3f {
        self.gravity
    }

    /// Get the time for a single physics update step
    pub fn delta_time(&self) -> f32 {
        self.delta_time
    }
}
