use crate::update::PhysicsData;
use alexandria::math::Vector3f;

impl PhysicsData {
    /// Create a new instance of [`PhysicsData`] with default values
    pub(in crate::update) fn new() -> PhysicsData {
        PhysicsData {
            gravity: Vector3f::new(0.0, -9.81, 0.0),
            delta_time: 1.0 / 60.0,
        }
    }
}
