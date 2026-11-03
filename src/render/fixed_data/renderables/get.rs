use crate::render::Renderables;

impl<T> Renderables<T> {
    /// Is the list of renderable items for this material type empty?
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
