use crate::render::LoadingMesh;

impl LoadingMesh {
    /// Is the mesh transfer complete?
    pub fn is_complete(&self) -> bool {
        self.receiver.is_available()
    }
}
