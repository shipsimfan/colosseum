use crate::render::{Camera, CameraProjection};
use alexandria::math::{Quaternionf, Vector3f};

impl Default for Camera {
    fn default() -> Self {
        Camera::new(
            Vector3f::ZERO,
            Quaternionf::IDENTITY,
            CameraProjection::default(),
        )
    }
}
