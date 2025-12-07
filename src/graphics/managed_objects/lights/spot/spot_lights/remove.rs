use crate::graphics::{SpotLightHandle, SpotLights};

impl SpotLights {
    /// Remove the [`SpotLight`](crate::graphics::SpotLight) identified by `handle`
    pub fn remove(&mut self, handle: SpotLightHandle) {
        self.list.remove(handle);
    }
}
