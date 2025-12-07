use crate::graphics::PointLights;

impl PointLights {
    /// Remove all registered point lights
    pub(crate) fn clear(&mut self) {
        self.list.clear();
    }
}
