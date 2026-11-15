use crate::physics::Ray;
use alexandria::math::Vector3f;

impl Ray {
    /// Create a new [`Ray`]
    pub fn new<O: Into<Vector3f>, D: Into<Vector3f>>(origin: O, direction: D) -> Ray {
        Ray {
            origin: origin.into(),
            direction: direction.into().normalized(),
        }
    }
}
