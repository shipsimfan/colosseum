use crate::update::Transform;
use alexandria::math::{Quaternionf, Vector3f};

impl Transform {
    /// Create a new [`Transform`]
    pub fn new<V1: Into<Vector3f>, Q: Into<Quaternionf>, V2: Into<Vector3f>>(
        position: V1,
        rotation: Q,
        scale: V2,
    ) -> Self {
        Transform {
            position: position.into(),
            rotation: rotation.into(),
            scale: scale.into(),
        }
    }
}
