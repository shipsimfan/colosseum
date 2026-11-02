use crate::render::Renderables;

impl<T> Renderables<T> {
    /// Clear all renderable items from the list
    pub fn reset(&mut self) {
        self.items.clear();
    }
}
