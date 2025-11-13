use crate::graphics::MeshRendererInner;

impl MeshRendererInner {
    /// Is this mesh renderer active?
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Enable this mesh renderer
    pub fn enable(&mut self) {
        self.active = true;
    }

    /// Disable this mesh renderer
    pub fn disable(&mut self) {
        self.active = false;
    }
}
