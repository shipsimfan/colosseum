use crate::graphics::{SpotLight, SpotLightHandle, SpotLights};

impl SpotLights {
    /// Get the [`SpotLight`] at `handle`
    pub fn get(&self, handle: SpotLightHandle) -> Option<&SpotLight> {
        self.list.get(handle)
    }

    /// Get the [`SpotLight`] at `handle` mutably
    pub fn get_mut(&mut self, handle: SpotLightHandle) -> Option<&mut SpotLight> {
        self.list.get_mut(handle)
    }
}
