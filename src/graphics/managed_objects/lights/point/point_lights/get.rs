use crate::graphics::{PointLight, PointLightHandle, PointLights};

impl PointLights {
    /// Get the [`PointLight`] at `handle`
    pub fn get(&self, handle: PointLightHandle) -> Option<&PointLight> {
        self.list.get(handle)
    }

    /// Get the [`PointLight`] at `handle` mutably
    pub fn get_mut(&mut self, handle: PointLightHandle) -> Option<&mut PointLight> {
        self.list.get_mut(handle)
    }
}
