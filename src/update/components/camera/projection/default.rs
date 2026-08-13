use crate::update::components::CameraProjection;

impl Default for CameraProjection {
    fn default() -> Self {
        CameraProjection::InfinitePerspective {
            fov_y: 70.0,
            near: 0.1,
        }
    }
}
