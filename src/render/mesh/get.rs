use crate::render::{Mesh, Vertex};

impl Mesh {
    /// Get the vertices of the mesh
    pub fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    /// Get the indices of the mesh
    pub fn indices(&self) -> &[u32] {
        &self.indices
    }
}
