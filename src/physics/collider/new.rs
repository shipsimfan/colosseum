use crate::physics::Collider;
use alexandria::math::Vector3f;

impl Collider {
    /// Create a new axis-aligned bounding box collider
    pub fn new_aabb<M: Into<Vector3f>, S: Into<Vector3f>>(min: M, size: S) -> Collider {
        Collider::AABB {
            min: min.into(),
            size: size.into(),
        }
    }
}
