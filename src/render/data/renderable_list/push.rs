use crate::{
    Result,
    render::{
        Material, Mesh,
        data::{RenderableList, renderable_list::RenderableBuffer},
    },
};
use alexandria::Id;

impl<T> RenderableList<T> {
    /// Push a new renderable object to the list
    pub fn push(&mut self, material: Id<Material>, mesh: Id<Mesh>, object_data: T) -> Result<()> {
        let buffer_index = self.count / RenderableBuffer::<T>::SIZE;

        if buffer_index >= self.buffers.len() {
            self.buffers.push(RenderableBuffer::<T>::new(
                &self.device,
                &self.memory_properties,
            )?);
        }

        self.buffers[buffer_index].push(material, mesh, object_data);
        self.count += 1;
        Ok(())
    }
}
