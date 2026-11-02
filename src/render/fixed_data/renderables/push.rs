use crate::render::{Mesh, Renderables};
use alexandria::Id;

impl<T> Renderables<T> {
    /// Add a new renderable item to the list
    pub fn push(&mut self, material: Id<T>, mesh: Id<Mesh>, index: usize) {
        self.items.push((material, mesh, index));
    }
}
