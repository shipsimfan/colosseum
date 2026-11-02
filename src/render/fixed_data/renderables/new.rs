use crate::render::Renderables;

impl<T> Renderables<T> {
    /// Create a new list of renderable items
    pub fn new() -> Renderables<T> {
        Renderables { items: Vec::new() }
    }
}
