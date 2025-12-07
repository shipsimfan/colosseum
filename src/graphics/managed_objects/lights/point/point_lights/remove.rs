use crate::graphics::{PointLightHandle, PointLights};

impl PointLights {
    /// Remove the [`PointLight`](crate::graphics::PointLight) identified by `handle`
    pub fn remove(&mut self, handle: PointLightHandle) {
        self.list.remove(handle);
    }
}
