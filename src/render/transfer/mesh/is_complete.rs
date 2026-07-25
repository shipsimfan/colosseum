use crate::render::MeshTransfer;

impl MeshTransfer {
    /// Is the mesh transfer complete?
    pub fn is_complete(&self) -> bool {
        self.receiver.is_available()
    }
}
