use crate::{
    Result,
    graphics::{Camera, CameraProjection, GraphicsContext},
};

impl GraphicsContext {
    /// Creates a new [`Camera`]
    pub fn create_camera(&self, projection: CameraProjection) -> Result<Camera> {
        self.managed_objects
            .create_camera(projection, self.size, &self.device)
    }
}
