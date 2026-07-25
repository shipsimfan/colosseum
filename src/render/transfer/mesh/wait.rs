use crate::{Result, render::MeshTransfer};

impl MeshTransfer {
    /// Wait for the mesh transfer to complete
    pub fn wait(&self) -> Result<()> {
        self.receiver.wait_no_take()
    }
}
