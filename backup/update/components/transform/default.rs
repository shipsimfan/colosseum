use crate::update::components::Transform;
use alexandria::math::{Quaternionf, Vector3f};

impl Default for Transform {
    fn default() -> Self {
        Transform::new(Vector3f::ZERO, Quaternionf::IDENTITY, Vector3f::ONE)
    }
}
