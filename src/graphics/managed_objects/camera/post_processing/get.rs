use crate::{graphics::CameraPostProcessing, math::Vector2u};

impl CameraPostProcessing {
    /// Get the size the objects for rendering are
    pub(in crate::graphics::managed_objects::camera) fn render_size(&self) -> Vector2u {
        self.render_scale_objects.render_size()
    }
}
