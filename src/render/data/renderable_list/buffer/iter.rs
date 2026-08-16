use crate::render::{Material, Mesh, data::renderable_list::RenderableBuffer};
use alexandria::{Id, gpu::GpuAddress};

impl<T> RenderableBuffer<T> {
    /// Get an iterator over the renderable objects in the buffer
    pub fn iter(&self) -> impl Iterator<Item = (Id<Material>, Id<Mesh>, GpuAddress<T>)> {
        self.renderables.iter().copied()
    }
}
