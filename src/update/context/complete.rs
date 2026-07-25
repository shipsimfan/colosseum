use crate::{
    render::{Mesh, MeshTransfer},
    update::UpdateContext,
};
use alexandria::Id;

impl<'a, Game: crate::Game> UpdateContext<'a, Game> {
    /// Complete the transfer of a mesh to the GPU
    pub fn complete_mesh(&mut self, transfer: MeshTransfer) -> Id<Mesh> {
        let (mesh, render_mesh, allocation) = transfer.take();
        self.render_data.add_render_object_change(render_mesh);
        self.render_objects.complete_mesh(mesh, allocation)
    }
}
