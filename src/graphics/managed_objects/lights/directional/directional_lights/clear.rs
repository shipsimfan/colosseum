use crate::graphics::DirectionalLights;

impl DirectionalLights {
    /// Remove all registered directional lights
    pub(crate) fn clear(&mut self) {
        self.list.clear();
    }
}
