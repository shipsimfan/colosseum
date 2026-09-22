use crate::update::PhysicsData;
use alexandria::math::Vector3f;

impl PhysicsData {
    /// Set the current force of gravity
    pub fn set_gravity<V: Into<Vector3f>>(&mut self, gravity: V) {
        self.gravity = gravity.into();
    }

    /// Set the time for a single physics update step
    pub fn set_delta_time(&mut self, delta_time: f32) {
        self.delta_time = delta_time;
    }

    /// Set the frame rate for the physics engine
    pub fn set_frame_rate(&mut self, frame_rate: f32) {
        self.delta_time = 1.0 / frame_rate;
    }
}
