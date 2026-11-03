use crate::render::{Renderable, Renderables};

impl<T> Renderables<T> {
    /// Get an iterator over the renderable items
    pub fn iter(&self) -> impl Iterator<Item = Renderable<T>> {
        self.items.iter().copied()
    }
}
