use crate::physics::Ray;
use alexandria::math::Vector3f;

impl Ray {
    /// Set the origin of the ray
    pub fn set_origin<O: Into<Vector3f>>(&mut self, origin: O) {
        self.origin = origin.into();
    }

    /// Set the direction of the ray
    pub fn set_direction<D: Into<Vector3f>>(&mut self, direction: D) {
        self.direction = direction.into().normalized();
    }
}
