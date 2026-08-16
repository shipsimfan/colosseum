use crate::render::{Material, Mesh, data::renderable_list::RenderableBuffer};
use alexandria::Id;

impl<T> RenderableBuffer<T> {
    /// Push a new renderable object to the list
    pub fn push(&mut self, material: Id<Material>, mesh: Id<Mesh>, object_data: T) {
        self.memory[self.renderables.len()] = object_data;
        let address = self.base_address.add(self.renderables.len());
        self.renderables.push((material, mesh, address));
    }
}
