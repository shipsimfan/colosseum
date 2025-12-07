use crate::{
    Result,
    graphics::{Camera, CameraHandle, CameraProjection, Cameras},
};

impl Cameras {
    /// Create a new [`Camera`]
    pub fn create(&mut self, projection: CameraProjection) -> Result<CameraHandle> {
        Ok(self.arena.insert(Camera::new(projection, &self.device)?))
    }
}
