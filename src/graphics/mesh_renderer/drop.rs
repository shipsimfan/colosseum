use crate::graphics::MeshRenderer;
use std::rc::Rc;

impl Drop for MeshRenderer {
    fn drop(&mut self) {
        // Is this the last reference to the mesh renderer, other than from the material?
        if Rc::strong_count(&self.mesh_renderer) == 2 {
            // If so, remove it from the material
            let mesh_renderer = self.mesh_renderer.borrow();
            mesh_renderer
                .material()
                .borrow_mut()
                .remove_mesh_renderer(&self.mesh_renderer);
        }
    }
}
