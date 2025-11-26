use crate::graphics::{MeshInner, Vertex};

impl MeshInner {
    /// Get the indices which define the faces of the mesh
    pub fn indices(&self) -> &[u32] {
        &self.indices
    }

    /// Get the vertices that make up this mesh
    pub fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }
}
