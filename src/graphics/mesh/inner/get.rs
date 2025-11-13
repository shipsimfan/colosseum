use crate::graphics::MeshInner;

impl MeshInner {
    /// Get the number of indices that make up the mesh
    pub(in crate::graphics) fn index_count(&self) -> u32 {
        self.index_count
    }
}
