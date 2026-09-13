use crate::update::components::CameraProjection;

impl CameraProjection {
    /// Get the depth of the camera's view frustum
    pub fn depth(&self, aspect: f32) -> f32 {
        match self {
            &CameraProjection::Perspective { near, far, fov_y } => {
                let half_width = (far - near) * (fov_y / 2.0).tan();
                let half_height = half_width * aspect;
                (half_width * half_width + half_height * half_height).sqrt()
            }
            &CameraProjection::InfinitePerspective {
                near,
                shadow_distance,
                fov_y,
            } => {
                let half_width = (shadow_distance - near) * (fov_y / 2.0).tan();
                let half_height = half_width * aspect;
                (half_width * half_width + half_height * half_height).sqrt()
            }
            &CameraProjection::Orthographic { near, far, .. } => far - near,
        }
    }
}
