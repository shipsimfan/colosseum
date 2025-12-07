use crate::graphics::MeshRenderers;

impl MeshRenderers {
    /// Remove all registered mesh renderers
    pub(crate) fn clear(&mut self) {
        self.arena.clear();
    }
}
