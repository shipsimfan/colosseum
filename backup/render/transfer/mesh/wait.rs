use crate::{Result, render::MeshTransfer};

impl MeshTransfer {
    /// Wait for the transfer to complete
    pub(in crate::render) fn wait(&mut self) -> Result<()> {
        self.receiver.wait_no_take()
    }
}
