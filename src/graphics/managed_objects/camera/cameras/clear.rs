use crate::graphics::Cameras;

impl Cameras {
    /// Remove all registered cameras
    pub(crate) fn clear(&mut self) {
        self.arena.clear();
    }
}
