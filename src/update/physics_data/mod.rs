use alexandria::math::Vector3f;

mod get;
mod new;
mod set;

/// Data describing the physics engine and interations
pub struct PhysicsData {
    /// The current force of gravity
    gravity: Vector3f,

    /// The time for a single physics update step
    delta_time: f32,
}
