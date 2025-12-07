use crate::Transforms;

impl Transforms {
    /// Remove all registered transforms
    pub(crate) fn clear(&mut self) {
        self.arena.clear();
    }
}
