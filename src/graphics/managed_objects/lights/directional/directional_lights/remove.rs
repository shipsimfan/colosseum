use crate::graphics::{DirectionalLightHandle, DirectionalLights};

impl DirectionalLights {
    /// Remove the [`DirectionalLight`](crate::graphics::DirectionalLight) identified by `handle`
    pub fn remove(&mut self, handle: DirectionalLightHandle) {
        self.list.remove(handle);
    }
}
