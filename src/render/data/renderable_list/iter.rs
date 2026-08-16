use crate::render::{
    Material, Mesh,
    data::{RenderableList, renderable_list::RenderableBuffer},
};
use alexandria::{Id, gpu::GpuAddress};

impl<T> RenderableList<T> {
    /// Get an iterator over the renderable objects in the list
    pub fn iter(&self) -> impl Iterator<Item = (Id<Material>, Id<Mesh>, GpuAddress<T>)> {
        self.buffers.iter().flat_map(RenderableBuffer::iter)
    }
}
