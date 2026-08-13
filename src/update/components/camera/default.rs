use crate::update::components::{Camera, CameraProjection};

impl Default for Camera {
    fn default() -> Self {
        Camera::new(CameraProjection::default())
    }
}
