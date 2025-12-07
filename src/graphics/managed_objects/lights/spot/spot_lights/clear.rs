use crate::graphics::SpotLights;

impl SpotLights {
    /// Remove all registered spot lights
    pub(crate) fn clear(&mut self) {
        self.list.clear();
    }
}
