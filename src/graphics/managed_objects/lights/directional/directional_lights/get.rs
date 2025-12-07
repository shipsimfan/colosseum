use crate::graphics::{DirectionalLight, DirectionalLightHandle, DirectionalLights};

impl DirectionalLights {
    /// Get the [`DirectionalLight`] at `handle`
    pub fn get(&self, handle: DirectionalLightHandle) -> Option<&DirectionalLight> {
        self.list.get(handle)
    }

    /// Get the [`DirectionalLight`] at `handle` mutably
    pub fn get_mut(&mut self, handle: DirectionalLightHandle) -> Option<&mut DirectionalLight> {
        self.list.get_mut(handle)
    }
}
